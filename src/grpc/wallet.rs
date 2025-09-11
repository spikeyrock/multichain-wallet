use tonic::{Request, Response, Status};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use super::auth::check_auth;
use super::wallet_proto::{
    wallet_service_server::WalletService as GrpcWalletService,
    GenerateWalletRequest, GenerateWalletResponse,
    BatchGenerateWalletRequest, BatchGenerateWalletResponse,
    WalletAddressResponse, MultipleWalletsResponse,
    TokenInfo, generate_wallet_response::Response as WalletResponseType,
};
use crate::services::wallet::WalletService;
use crate::core::{get_chain_info, get_chain_types_by_symbol, get_token_registry};

pub struct WalletServiceImpl {
    wallet_service: Arc<Mutex<WalletService>>,
}

impl WalletServiceImpl {
    pub fn new(wallet_service: Arc<Mutex<WalletService>>) -> Self {
        Self { wallet_service }
    }
    
    fn get_chain_identifier(chain_type: &crate::core::ChainType) -> &'static str {
        match chain_type {
            crate::core::ChainType::Ethereum => "Ethereum",
            crate::core::ChainType::Base => "Base",
            crate::core::ChainType::Arbitrum => "Arbitrum",
            crate::core::ChainType::Optimism => "Optimism",
            crate::core::ChainType::Polygon => "Polygon",
            crate::core::ChainType::Avalanche => "Avalanche",
            crate::core::ChainType::BitcoinSegwit => "Bitcoin",
            crate::core::ChainType::BitcoinLegacy => "Bitcoin",
            crate::core::ChainType::BitcoinTaproot => "Bitcoin",
            crate::core::ChainType::Solana => "Solana",
            crate::core::ChainType::Tron => "Tron",
            crate::core::ChainType::Ripple => "Ripple",
            crate::core::ChainType::Sui => "Sui",
            crate::core::ChainType::Near => "Near",
            crate::core::ChainType::Dogecoin => "Dogecoin",
            crate::core::ChainType::Cosmos => "Cosmos",
            crate::core::ChainType::Osmosis => "Osmosis",
            crate::core::ChainType::Juno => "Juno",
            crate::core::ChainType::Secret => "Secret",
            crate::core::ChainType::Akash => "Akash",
            crate::core::ChainType::Sei => "Sei",
            crate::core::ChainType::Celestia => "Celestia",
            crate::core::ChainType::Injective => "Injective",
            crate::core::ChainType::Tezos => "Tezos",
            crate::core::ChainType::Filecoin => "Filecoin",
        }
    }
    
    fn get_supported_tokens(chain_type: &crate::core::ChainType) -> Vec<TokenInfo> {
        let registry = get_token_registry();
        let chain_identifier = Self::get_chain_identifier(chain_type);
        let chain_tokens = registry.get_tokens_by_chain(chain_identifier);
        
        chain_tokens
            .into_iter()
            .filter_map(|token| {
                token.deployments.iter()
                    .find(|d| d.chain == chain_identifier || d.chain_type == chain_identifier)
                    .map(|deployment| TokenInfo {
                        symbol: deployment.symbol.clone(),
                        name: token.name.clone(),
                        contract_address: deployment.contract_address.clone().unwrap_or_default(),
                        decimals: deployment.decimals as u32,
                        token_standard: deployment.token_standard.clone(),
                        is_native: deployment.is_native,
                    })
            })
            .collect()
    }
    
    fn convert_wallet_to_grpc(wallet: crate::core::WalletAddress) -> WalletAddressResponse {
        let supported_tokens = Self::get_supported_tokens(&wallet.chain_type);
        
        WalletAddressResponse {
            address: wallet.address,
            chain_name: wallet.chain_info.name,
            chain_symbol: wallet.chain_info.symbol,
            address_type: format!("{:?}", wallet.chain_type),
            derivation_path: wallet.derivation_path,
            index: wallet.index,
            public_key: wallet.public_key,
            private_key: wallet.private_key,
            supported_tokens,
        }
    }
}

#[tonic::async_trait]
impl GrpcWalletService for WalletServiceImpl {
    async fn generate_wallet(
        &self,
        request: Request<GenerateWalletRequest>,
    ) -> Result<Response<GenerateWalletResponse>, Status> {
        check_auth(&request)?;
        let req = request.into_inner();
        
        // Get chain types for the symbol
        let chain_types = get_chain_types_by_symbol(&req.symbol);
        
        if chain_types.is_empty() {
            return Err(Status::invalid_argument(
                format!("Unsupported symbol: {}", req.symbol)
            ));
        }
        
        let mut service = self.wallet_service.lock().await;
        let mut wallets = Vec::new();
        
        // Generate wallets for all chain types associated with the symbol
        for chain_type in chain_types {
            let chain_info = get_chain_info(&chain_type);
            
            info!(
                "Generating {} ({}) wallet at index {}",
                chain_info.name, chain_info.symbol, req.index
            );
            
            let wallet = service
                .generate_wallet_address(
                    &req.mnemonic,
                    &req.passphrase,
                    &chain_type,
                    req.index,
                )
                .await
                .map_err(|e| Status::internal(format!("Failed to generate wallet: {}", e)))?;
                
            wallets.push(wallet);
        }
        
        let response = if wallets.len() == 1 {
            // Single wallet response
            let wallet = wallets.into_iter().next().unwrap();
            let grpc_wallet = Self::convert_wallet_to_grpc(wallet);
            
            GenerateWalletResponse {
                response: Some(WalletResponseType::SingleWallet(grpc_wallet)),
            }
        } else {
            // Multiple wallets response (e.g., Bitcoin with different address types)
            let grpc_wallets: Vec<WalletAddressResponse> = wallets
                .into_iter()
                .map(Self::convert_wallet_to_grpc)
                .collect();
                
            GenerateWalletResponse {
                response: Some(WalletResponseType::MultipleWallets(MultipleWalletsResponse {
                    wallets: grpc_wallets,
                })),
            }
        };

        Ok(Response::new(response))
    }

    async fn batch_generate_wallets(
        &self,
        request: Request<BatchGenerateWalletRequest>,
    ) -> Result<Response<BatchGenerateWalletResponse>, Status> {
        check_auth(&request)?;
        let req = request.into_inner();
        
        // Validate count
        if req.count == 0 || req.count > 100 {
            return Err(Status::invalid_argument(
                "Count must be between 1 and 100".to_string(),
            ));
        }

        if req.symbols.is_empty() {
            return Err(Status::invalid_argument(
                "At least one symbol must be specified".to_string(),
            ));
        }

        // Convert symbols to chain types
        let mut all_chain_types = Vec::new();
        for symbol in &req.symbols {
            let chain_types = get_chain_types_by_symbol(symbol);
            if chain_types.is_empty() {
                return Err(Status::invalid_argument(
                    format!("Unsupported symbol: {}", symbol)
                ));
            }
            all_chain_types.extend(chain_types);
        }

        info!(
            "Batch generating {} addresses for {} chain types starting at index {}",
            req.count,
            all_chain_types.len(),
            req.start_index
        );

        let mut service = self.wallet_service.lock().await;
        let addresses = service
            .batch_generate_wallet_addresses(
                &req.mnemonic,
                &req.passphrase,
                &all_chain_types,
                req.start_index,
                req.count,
            )
            .await
            .map_err(|e| Status::internal(format!("Failed to batch generate wallets: {}", e)))?;

        let response_addresses: Vec<WalletAddressResponse> = addresses
            .into_iter()
            .map(Self::convert_wallet_to_grpc)
            .collect();

        let response = BatchGenerateWalletResponse {
            addresses: response_addresses,
        };

        Ok(Response::new(response))
    }
}