use actix_web::{get, post, web, HttpResponse};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use crate::api::models::*;
use crate::core::{get_chain_info, get_chain_types_by_symbol, get_token_registry};
use crate::errors::{ApiError, ApiResult};
use crate::services::wallet::WalletService;

#[get("/health")]
pub async fn health_check() -> ApiResult<HttpResponse> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now().timestamp(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[post("/mnemonic/generate")]
pub async fn generate_mnemonic(
    wallet_service: web::Data<Arc<Mutex<WalletService>>>,
    req: web::Json<GenerateMnemonicRequest>,
) -> ApiResult<HttpResponse> {
    // Validate request
    req.validate()
        .map_err(|e| ApiError::BadRequest(e))?;

    info!(
        "Generating {} word mnemonic in {}",
        req.word_count, req.language
    );

    // Generate mnemonic
    let service = wallet_service.lock().await;
    let mnemonic = service
        .generate_mnemonic(&req.language, req.word_count)
        .await?;

    let response = GenerateMnemonicResponse {
        mnemonic,
        language: req.language.clone(),
        word_count: req.word_count,
        generated_at: chrono::Utc::now().timestamp(),
    };

    Ok(HttpResponse::Ok().json(response))
}

#[post("/mnemonic/validate")]
pub async fn validate_mnemonic(
    wallet_service: web::Data<Arc<Mutex<WalletService>>>,
    req: web::Json<ValidateMnemonicRequest>,
) -> ApiResult<HttpResponse> {
    info!("Validating mnemonic in {}", req.language);

    let service = wallet_service.lock().await;
    let (valid, word_count) = service
        .validate_mnemonic(&req.mnemonic, &req.language)
        .await;

    let response = ValidateMnemonicResponse {
        valid,
        word_count: if valid { Some(word_count) } else { None },
        message: if valid {
            Some("Valid mnemonic phrase".to_string())
        } else {
            Some("Invalid mnemonic phrase".to_string())
        },
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/languages")]
pub async fn get_supported_languages() -> ApiResult<HttpResponse> {
    let languages = vec![
        LanguageInfo {
            code: "english".to_string(),
            name: "English".to_string(),
            native_name: "English".to_string(),
        },
        LanguageInfo {
            code: "japanese".to_string(),
            name: "Japanese".to_string(),
            native_name: "日本語".to_string(),
        },
        LanguageInfo {
            code: "korean".to_string(),
            name: "Korean".to_string(),
            native_name: "한국어".to_string(),
        },
        LanguageInfo {
            code: "spanish".to_string(),
            name: "Spanish".to_string(),
            native_name: "Español".to_string(),
        },
        LanguageInfo {
            code: "chinese_simplified".to_string(),
            name: "Chinese (Simplified)".to_string(),
            native_name: "中文(简体)".to_string(),
        },
        LanguageInfo {
            code: "chinese_traditional".to_string(),
            name: "Chinese (Traditional)".to_string(),
            native_name: "中文(繁體)".to_string(),
        },
        LanguageInfo {
            code: "french".to_string(),
            name: "French".to_string(),
            native_name: "Français".to_string(),
        },
        LanguageInfo {
            code: "italian".to_string(),
            name: "Italian".to_string(),
            native_name: "Italiano".to_string(),
        },
        LanguageInfo {
            code: "czech".to_string(),
            name: "Czech".to_string(),
            native_name: "Čeština".to_string(),
        },
        LanguageInfo {
            code: "portuguese".to_string(),
            name: "Portuguese".to_string(),
            native_name: "Português".to_string(),
        },
    ];

    let response = SupportedLanguagesResponse { languages };

    Ok(HttpResponse::Ok().json(response))
}

#[post("/wallet/generate")]
pub async fn generate_wallet(
    wallet_service: web::Data<Arc<Mutex<WalletService>>>,
    req: web::Json<GenerateWalletRequest>,
) -> ApiResult<HttpResponse> {
    // Get chain types for the symbol
    let chain_types = get_chain_types_by_symbol(&req.symbol);
    
    if chain_types.is_empty() {
        return Err(ApiError::BadRequest(
            format!("Unsupported symbol: {}", req.symbol)
        ));
    }
    
    let mut service = wallet_service.lock().await;
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
            .await?;
            
        wallets.push(wallet);
    }
    
    let registry = get_token_registry();
    
    // Helper function to get chain identifier
    let get_chain_identifier = |chain_type: &crate::core::ChainType| -> &'static str {
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
    };
    
    // Helper function to get supported tokens for a chain
    let get_supported_tokens = |chain_type: &crate::core::ChainType| -> Option<Vec<TokenInfo>> {
        let chain_identifier = get_chain_identifier(chain_type);
        let chain_tokens = registry.get_tokens_by_chain(chain_identifier);
        let tokens: Vec<TokenInfo> = chain_tokens
            .into_iter()
            .filter_map(|token| {
                token.deployments.iter()
                    .find(|d| d.chain == chain_identifier || d.chain_type == chain_identifier)
                    .map(|deployment| TokenInfo {
                        symbol: deployment.symbol.clone(),
                        name: token.name.clone(),
                        contract_address: deployment.contract_address.clone(),
                        decimals: deployment.decimals,
                        token_standard: deployment.token_standard.clone(),
                    })
            })
            .collect();
        
        if tokens.is_empty() { None } else { Some(tokens) }
    };
    
    // If only one wallet (most coins), return single response
    if wallets.len() == 1 {
        let wallet = wallets.into_iter().next().unwrap();
        let response = GenerateWalletResponse {
            address: wallet.address,
            chain_name: wallet.chain_info.name,
            chain_symbol: wallet.chain_info.symbol,
            address_type: wallet.chain_type.clone().into(),
            derivation_path: wallet.derivation_path,
            index: wallet.index,
            public_key: wallet.public_key,
            private_key: wallet.private_key,
            supported_tokens: get_supported_tokens(&wallet.chain_type),
        };
        Ok(HttpResponse::Ok().json(response))
    } else {
        // For multiple wallets (BTC), return array
        let responses: Vec<GenerateWalletResponse> = wallets
            .into_iter()
            .map(|wallet| {
                GenerateWalletResponse {
                    address: wallet.address.clone(),
                    chain_name: wallet.chain_info.name,
                    chain_symbol: wallet.chain_info.symbol,
                    address_type: wallet.chain_type.clone().into(),
                    derivation_path: wallet.derivation_path,
                    index: wallet.index,
                    public_key: wallet.public_key,
                    private_key: wallet.private_key,
                    supported_tokens: get_supported_tokens(&wallet.chain_type),
                }
            })
            .collect();
        Ok(HttpResponse::Ok().json(responses))
    }
}

#[post("/wallet/batch")]
pub async fn batch_generate_wallets(
    wallet_service: web::Data<Arc<Mutex<WalletService>>>,
    req: web::Json<BatchGenerateWalletRequest>,
) -> ApiResult<HttpResponse> {
    // Validate count
    if req.count == 0 || req.count > 100 {
        return Err(ApiError::BadRequest(
            "Count must be between 1 and 100".to_string(),
        ));
    }

    if req.symbols.is_empty() {
        return Err(ApiError::BadRequest(
            "At least one symbol must be specified".to_string(),
        ));
    }

    // Convert symbols to chain types
    let mut all_chain_types = Vec::new();
    for symbol in &req.symbols {
        let chain_types = get_chain_types_by_symbol(symbol);
        if chain_types.is_empty() {
            return Err(ApiError::BadRequest(
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

    let mut service = wallet_service.lock().await;
    let addresses = service
        .batch_generate_wallet_addresses(
            &req.mnemonic,
            &req.passphrase,
            &all_chain_types,
            req.start_index,
            req.count,
        )
        .await?;

    let registry = get_token_registry();
    
    let response_addresses: Vec<WalletAddressResponse> = addresses
        .into_iter()
        .map(|wallet| {
            // Get the chain identifier (convert chain type to string)
            let chain_identifier = match &wallet.chain_type {
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
            };
            
            // Get all tokens supported on this chain
            let chain_tokens = registry.get_tokens_by_chain(chain_identifier);
            let supported_tokens: Vec<TokenInfo> = chain_tokens
                .into_iter()
                .filter_map(|token| {
                    // Find the deployment for this specific chain
                    token.deployments.iter()
                        .find(|d| d.chain == chain_identifier || d.chain_type == chain_identifier)
                        .map(|deployment| TokenInfo {
                            symbol: deployment.symbol.clone(),
                            name: token.name.clone(),
                            contract_address: deployment.contract_address.clone(),
                            decimals: deployment.decimals,
                            token_standard: deployment.token_standard.clone(),
                        })
                })
                .collect();
            
            WalletAddressResponse {
                address: wallet.address.clone(),
                chain_name: wallet.chain_info.name,
                chain_symbol: wallet.chain_info.symbol,
                address_type: wallet.chain_type.into(),
                derivation_path: wallet.derivation_path,
                index: wallet.index,
                public_key: wallet.public_key,
                private_key: wallet.private_key,
                supported_tokens: if supported_tokens.is_empty() { None } else { Some(supported_tokens) },
            }
        })
        .collect();

    let response = BatchGenerateWalletResponse {
        addresses: response_addresses,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/wallet/types")]
pub async fn get_supported_wallet_types(
    wallet_service: web::Data<Arc<Mutex<WalletService>>>,
) -> ApiResult<HttpResponse> {
    let service = wallet_service.lock().await;
    let chains = service.list_supported_chains().await;
    
    // Group chains by symbol
    let mut symbols_map: std::collections::HashMap<String, Vec<serde_json::Value>> = std::collections::HashMap::new();
    
    let total_chains = chains.len();
    
    for chain in chains {
        let chain_type = match chain.name.as_str() {
            "Bitcoin" => {
                // Determine Bitcoin variant based on address format
                match &chain.address_format {
                    crate::core::AddressFormat::Bech32 { hrp } if hrp == "bc" => "bitcoin_segwit",
                    crate::core::AddressFormat::Bitcoin { .. } => "bitcoin_legacy",
                    _ => "bitcoin_taproot",
                }
            },
            "Ethereum" => "ethereum",
            "Ripple" => "xrp",
            "Solana" => "solana",
            "TRON" => "tron",
            "Sui" => "sui",
            "NEAR Protocol" => "near",
            "Dogecoin" => "dogecoin",
            "Cosmos" => "cosmos",
            "Osmosis" => "osmosis",
            "Juno" => "juno",
            "Secret Network" => "secret",
            "Akash" => "akash",
            "Sei" => "sei",
            "Celestia" => "celestia",
            "Injective" => "injective",
            "Tezos" => "tezos",
            "Filecoin" => "filecoin",
            _ => "unknown"
        };
        
        let chain_detail = serde_json::json!({
            "type": chain_type,
            "name": chain.name,
            "symbol": chain.symbol,
            "decimals": chain.decimals,
            "description": format!("{} ({}) - {} decimals", chain.name, chain.symbol, chain.decimals)
        });
        
        symbols_map.entry(chain.symbol.clone())
            .or_insert_with(Vec::new)
            .push(chain_detail);
    }
    
    // Get list of supported symbols
    let supported_symbols: Vec<&str> = vec![
        "BTC", "ETH", "XRP", "SOL", "TRX", "SUI", "NEAR", "DOGE",
        "ATOM", "OSMO", "JUNO", "SCRT", "AKT", "SEI", "TIA", "INJ", "XTZ", "FIL",
    ];
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "supported_symbols": supported_symbols,
        "chains_by_symbol": symbols_map,
        "total_chains": total_chains
    })))
}

