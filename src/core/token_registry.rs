/// Complete Unified Token Registry
/// This module contains ALL tokens from the unified registry properly categorized
/// and mapped to their respective chains

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainDeployment {
    pub chain: String,
    pub chain_type: String,
    pub chain_id: Option<u32>,
    pub contract_address: Option<String>,
    pub decimals: u8,
    pub symbol: String,
    pub is_native: bool,
    pub token_standard: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
    Native,
    Token,
    Wrapped,
    Stablecoin,
    Synthetic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedToken {
    pub base_symbol: String,
    pub name: String,
    pub cmc_id: Option<u32>,
    pub coingecko_id: Option<String>,
    pub category: String,
    pub is_multi_chain: bool,
    pub asset_type: AssetType,
    pub deployments: Vec<ChainDeployment>,
}

pub struct CompleteTokenRegistry {
    tokens: HashMap<String, UnifiedToken>,
}

impl CompleteTokenRegistry {
    pub fn new() -> Self {
        let mut registry = CompleteTokenRegistry {
            tokens: HashMap::new(),
        };
        
        // Initialize all token categories
        registry.initialize_native_tokens();
        registry.initialize_stablecoins();
        registry.initialize_wrapped_tokens();
        registry.initialize_defi_tokens();
        registry.initialize_layer2_tokens();
        registry.initialize_solana_ecosystem();
        registry.initialize_cosmos_ecosystem();
        
        registry
    }
    
    /// Initialize all native blockchain tokens
    fn initialize_native_tokens(&mut self) {
        // Bitcoin - with all address type variants
        self.add_token(UnifiedToken {
            base_symbol: "BTC".to_string(),
            name: "Bitcoin".to_string(),
            cmc_id: Some(1),
            coingecko_id: Some("bitcoin".to_string()),
            category: "Layer 1".to_string(),
            is_multi_chain: false,
            asset_type: AssetType::Native,
            deployments: vec![
                ChainDeployment {
                    chain: "Bitcoin".to_string(),
                    chain_type: "BitcoinLegacy".to_string(),
                    chain_id: None,
                    contract_address: None,
                    decimals: 8,
                    symbol: "BTC".to_string(),
                    is_native: true,
                    token_standard: "Native".to_string(),
                },
                ChainDeployment {
                    chain: "Bitcoin".to_string(),
                    chain_type: "BitcoinSegwit".to_string(),
                    chain_id: None,
                    contract_address: None,
                    decimals: 8,
                    symbol: "BTC".to_string(),
                    is_native: true,
                    token_standard: "Native".to_string(),
                },
                ChainDeployment {
                    chain: "Bitcoin".to_string(),
                    chain_type: "BitcoinTaproot".to_string(),
                    chain_id: None,
                    contract_address: None,
                    decimals: 8,
                    symbol: "BTC".to_string(),
                    is_native: true,
                    token_standard: "Native".to_string(),
                },
            ],
        });
        
        // Ethereum - Native on mainnet and all L2s
        self.add_token(UnifiedToken {
            base_symbol: "ETH".to_string(),
            name: "Ethereum".to_string(),
            cmc_id: Some(1027),
            coingecko_id: Some("ethereum".to_string()),
            category: "Layer 1".to_string(),
            is_multi_chain: true,
            asset_type: AssetType::Native,
            deployments: vec![
                ChainDeployment {
                    chain: "Ethereum".to_string(),
                    chain_type: "Ethereum".to_string(),
                    chain_id: Some(1),
                    contract_address: None,
                    decimals: 18,
                    symbol: "ETH".to_string(),
                    is_native: true,
                    token_standard: "Native".to_string(),
                },
                ChainDeployment {
                    chain: "Base".to_string(),
                    chain_type: "Base".to_string(),
                    chain_id: Some(8453),
                    contract_address: None,
                    decimals: 18,
                    symbol: "ETH".to_string(),
                    is_native: true,
                    token_standard: "Native".to_string(),
                },
                ChainDeployment {
                    chain: "Arbitrum".to_string(),
                    chain_type: "Arbitrum".to_string(),
                    chain_id: Some(42161),
                    contract_address: None,
                    decimals: 18,
                    symbol: "ETH".to_string(),
                    is_native: true,
                    token_standard: "Native".to_string(),
                },
                ChainDeployment {
                    chain: "Optimism".to_string(),
                    chain_type: "Optimism".to_string(),
                    chain_id: Some(10),
                    contract_address: None,
                    decimals: 18,
                    symbol: "ETH".to_string(),
                    is_native: true,
                    token_standard: "Native".to_string(),
                },
                // Wrapped ETH on non-native chains
                ChainDeployment {
                    chain: "Polygon".to_string(),
                    chain_type: "Polygon".to_string(),
                    chain_id: Some(137),
                    contract_address: Some("0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619".to_string()),
                    decimals: 18,
                    symbol: "WETH".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Solana".to_string(),
                    chain_type: "Solana".to_string(),
                    chain_id: Some(101),
                    contract_address: Some("7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs".to_string()),
                    decimals: 8,
                    symbol: "WETH".to_string(),
                    is_native: false,
                    token_standard: "SPL".to_string(),
                },
                ChainDeployment {
                    chain: "Avalanche".to_string(),
                    chain_type: "Avalanche".to_string(),
                    chain_id: Some(43114),
                    contract_address: Some("0x49D5c2BdFfac6CE2BFdB6640F4F80f226bc10bAB".to_string()),
                    decimals: 18,
                    symbol: "WETH.e".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                // Non-EVM chains
                ChainDeployment {
                    chain: "Solana".to_string(),
                    chain_type: "Solana".to_string(),
                    chain_id: Some(101),
                    contract_address: Some("7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs".to_string()),
                    decimals: 8,
                    symbol: "WETH".to_string(),
                    is_native: false,
                    token_standard: "SPL".to_string(),
                },
                ChainDeployment {
                    chain: "Tron".to_string(),
                    chain_type: "Tron".to_string(),
                    chain_id: None,
                    contract_address: Some("TXWkP3jLBqRGojUih1ShzNyDaN5Csnebok".to_string()),
                    decimals: 18,
                    symbol: "ETH".to_string(),
                    is_native: false,
                    token_standard: "TRC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Sui".to_string(),
                    chain_type: "Sui".to_string(),
                    chain_id: None,
                    contract_address: Some("0xaf8cd5edc19c4512f4259f0bee101a40d41ebed738ade5874359610ef8eeced5::coin::COIN".to_string()),
                    decimals: 8,
                    symbol: "WETH".to_string(),
                    is_native: false,
                    token_standard: "Sui Coin".to_string(),
                },
                ChainDeployment {
                    chain: "Near".to_string(),
                    chain_type: "Near".to_string(),
                    chain_id: None,
                    contract_address: Some("aurora".to_string()),
                    decimals: 18,
                    symbol: "ETH".to_string(),
                    is_native: false,
                    token_standard: "NEP-141".to_string(),
                },
                ChainDeployment {
                    chain: "Tezos".to_string(),
                    chain_type: "Tezos".to_string(),
                    chain_id: None,
                    contract_address: Some("KT1VG2WtYdSWz5E7chTeAdDPZNy2MpP8pTfL".to_string()),
                    decimals: 18,
                    symbol: "ETHtz".to_string(),
                    is_native: false,
                    token_standard: "FA1.2".to_string(),
                },
                ChainDeployment {
                    chain: "Filecoin".to_string(),
                    chain_type: "Filecoin".to_string(),
                    chain_id: Some(314),
                    contract_address: Some("0x60E1773636CF5E4A227d9AC24F20fEca034ee25A".to_string()),
                    decimals: 18,
                    symbol: "WETH".to_string(),
                    is_native: false,
                    token_standard: "FRC-20".to_string(),
                },
            ],
        });
        
        // Solana
        self.add_token(UnifiedToken {
            base_symbol: "SOL".to_string(),
            name: "Solana".to_string(),
            cmc_id: Some(5426),
            coingecko_id: Some("solana".to_string()),
            category: "Layer 1".to_string(),
            is_multi_chain: true,
            asset_type: AssetType::Native,
            deployments: vec![
                ChainDeployment {
                    chain: "Solana".to_string(),
                    chain_type: "Solana".to_string(),
                    chain_id: Some(101),
                    contract_address: None,
                    decimals: 9,
                    symbol: "SOL".to_string(),
                    is_native: true,
                    token_standard: "Native".to_string(),
                },
                ChainDeployment {
                    chain: "Ethereum".to_string(),
                    chain_type: "Ethereum".to_string(),
                    chain_id: Some(1),
                    contract_address: Some("0xD31a59c85aE9D8edEFeC411D448f90841571b89c".to_string()),
                    decimals: 9,
                    symbol: "SOL".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
            ],
        });
        
        // Polygon/MATIC
        self.add_token(UnifiedToken {
            base_symbol: "MATIC".to_string(),
            name: "Polygon".to_string(),
            cmc_id: Some(3890),
            coingecko_id: Some("matic-network".to_string()),
            category: "Layer 2".to_string(),
            is_multi_chain: true,
            asset_type: AssetType::Native,
            deployments: vec![
                ChainDeployment {
                    chain: "Polygon".to_string(),
                    chain_type: "Polygon".to_string(),
                    chain_id: Some(137),
                    contract_address: None,
                    decimals: 18,
                    symbol: "MATIC".to_string(),
                    is_native: true,
                    token_standard: "Native".to_string(),
                },
                ChainDeployment {
                    chain: "Ethereum".to_string(),
                    chain_type: "Ethereum".to_string(),
                    chain_id: Some(1),
                    contract_address: Some("0x7D1AfA7B718fb893dB30A3aBc0Cfc608AaCfeBB0".to_string()),
                    decimals: 18,
                    symbol: "MATIC".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Solana".to_string(),
                    chain_type: "Solana".to_string(),
                    chain_id: Some(101),
                    contract_address: Some("C7NNPWuZCNjZBfW5p6JvGsR8pUdsRpEdP1ZAhnoDwj7h".to_string()),
                    decimals: 8,
                    symbol: "MATIC".to_string(),
                    is_native: false,
                    token_standard: "SPL".to_string(),
                },
            ],
        });
        
        // Avalanche
        self.add_token(UnifiedToken {
            base_symbol: "AVAX".to_string(),
            name: "Avalanche".to_string(),
            cmc_id: Some(5805),
            coingecko_id: Some("avalanche-2".to_string()),
            category: "Layer 1".to_string(),
            is_multi_chain: true,
            asset_type: AssetType::Native,
            deployments: vec![
                ChainDeployment {
                    chain: "Avalanche".to_string(),
                    chain_type: "Avalanche".to_string(),
                    chain_id: Some(43114),
                    contract_address: None,
                    decimals: 18,
                    symbol: "AVAX".to_string(),
                    is_native: true,
                    token_standard: "Native".to_string(),
                },
                ChainDeployment {
                    chain: "Ethereum".to_string(),
                    chain_type: "Ethereum".to_string(),
                    chain_id: Some(1),
                    contract_address: Some("0x85f138bfEE4ef8e540890CFb48F620571d67Eda3".to_string()),
                    decimals: 18,
                    symbol: "AVAX".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
            ],
        });
        
        // Add other native tokens
        self.add_native_token("DOGE", "Dogecoin", 74, "dogecoin", "Dogecoin", 8);
        self.add_native_token("TRX", "TRON", 1958, "tron", "Tron", 6);
        self.add_native_token("FIL", "Filecoin", 2280, "filecoin", "Filecoin", 18);
        self.add_native_token("NEAR", "NEAR Protocol", 6535, "near", "Near", 24);
        self.add_native_token("SUI", "Sui", 20947, "sui", "Sui", 9);
        self.add_native_token("XRP", "XRP", 52, "ripple", "Ripple", 6);
        self.add_native_token("XTZ", "Tezos", 2011, "tezos", "Tezos", 6);
        self.add_native_token("LTC", "Litecoin", 2, "litecoin", "Litecoin", 8);
        self.add_native_token("BCH", "Bitcoin Cash", 1831, "bitcoin-cash", "BitcoinCash", 8);
        self.add_native_token("ADA", "Cardano", 2010, "cardano", "Cardano", 6);
    }
    
    /// Initialize all stablecoins with their multichain deployments
    fn initialize_stablecoins(&mut self) {
        // USDT - Tether (most widely deployed stablecoin)
        self.add_token(UnifiedToken {
            base_symbol: "USDT".to_string(),
            name: "Tether".to_string(),
            cmc_id: Some(825),
            coingecko_id: Some("tether".to_string()),
            category: "Stablecoin".to_string(),
            is_multi_chain: true,
            asset_type: AssetType::Stablecoin,
            deployments: vec![
                // EVM Chains
                ChainDeployment {
                    chain: "Ethereum".to_string(),
                    chain_type: "Ethereum".to_string(),
                    chain_id: Some(1),
                    contract_address: Some("0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string()),
                    decimals: 6,
                    symbol: "USDT".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Base".to_string(),
                    chain_type: "Base".to_string(),
                    chain_id: Some(8453),
                    contract_address: Some("0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2".to_string()),
                    decimals: 6,
                    symbol: "USDT".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Arbitrum".to_string(),
                    chain_type: "Arbitrum".to_string(),
                    chain_id: Some(42161),
                    contract_address: Some("0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9".to_string()),
                    decimals: 6,
                    symbol: "USDT".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Optimism".to_string(),
                    chain_type: "Optimism".to_string(),
                    chain_id: Some(10),
                    contract_address: Some("0x94b008aA00579c1307B0EF2c499aD98a8ce58e58".to_string()),
                    decimals: 6,
                    symbol: "USDT".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Polygon".to_string(),
                    chain_type: "Polygon".to_string(),
                    chain_id: Some(137),
                    contract_address: Some("0xc2132D05D31c914a87C6611C10748AEb04B58e8F".to_string()),
                    decimals: 6,
                    symbol: "USDT".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Avalanche".to_string(),
                    chain_type: "Avalanche".to_string(),
                    chain_id: Some(43114),
                    contract_address: Some("0x9702230A8Ea53601f5cD2dc00fDBc13d4dF4A8c7".to_string()),
                    decimals: 6,
                    symbol: "USDt".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                // Non-EVM Chains
                ChainDeployment {
                    chain: "Solana".to_string(),
                    chain_type: "Solana".to_string(),
                    chain_id: Some(101),
                    contract_address: Some("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB".to_string()),
                    decimals: 6,
                    symbol: "USDT".to_string(),
                    is_native: false,
                    token_standard: "SPL".to_string(),
                },
                ChainDeployment {
                    chain: "Tron".to_string(),
                    chain_type: "Tron".to_string(),
                    chain_id: None,
                    contract_address: Some("TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".to_string()),
                    decimals: 6,
                    symbol: "USDT".to_string(),
                    is_native: false,
                    token_standard: "TRC-20".to_string(),
                },
                // Additional chains
                ChainDeployment {
                    chain: "Sui".to_string(),
                    chain_type: "Sui".to_string(),
                    chain_id: None,
                    contract_address: Some("0xc060006111016b8a020ad5b33834984a437aaa7d3c74c18e09a95d48aceab08c::coin::COIN".to_string()),
                    decimals: 6,
                    symbol: "USDT".to_string(),
                    is_native: false,
                    token_standard: "Sui Coin".to_string(),
                },
                ChainDeployment {
                    chain: "Near".to_string(),
                    chain_type: "Near".to_string(),
                    chain_id: None,
                    contract_address: Some("usdt.tether-token.near".to_string()),
                    decimals: 6,
                    symbol: "USDT".to_string(),
                    is_native: false,
                    token_standard: "NEP-141".to_string(),
                },
                ChainDeployment {
                    chain: "Tezos".to_string(),
                    chain_type: "Tezos".to_string(),
                    chain_id: None,
                    contract_address: Some("KT1XnTn74bUtxHfDtBmm2bGZAQfhPbvKWR8o".to_string()),
                    decimals: 6,
                    symbol: "USDt".to_string(),
                    is_native: false,
                    token_standard: "FA1.2".to_string(),
                },
                ChainDeployment {
                    chain: "Filecoin".to_string(),
                    chain_type: "Filecoin".to_string(),
                    chain_id: Some(314),
                    contract_address: Some("0x422849b355039bc58f2780cc4854919fc9cfaf94".to_string()),
                    decimals: 6,
                    symbol: "USDT".to_string(),
                    is_native: false,
                    token_standard: "FRC-20".to_string(),
                },
            ],
        });
        
        // USDC - USD Coin
        self.add_token(UnifiedToken {
            base_symbol: "USDC".to_string(),
            name: "USD Coin".to_string(),
            cmc_id: Some(3408),
            coingecko_id: Some("usd-coin".to_string()),
            category: "Stablecoin".to_string(),
            is_multi_chain: true,
            asset_type: AssetType::Stablecoin,
            deployments: vec![
                // EVM Chains
                ChainDeployment {
                    chain: "Ethereum".to_string(),
                    chain_type: "Ethereum".to_string(),
                    chain_id: Some(1),
                    contract_address: Some("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string()),
                    decimals: 6,
                    symbol: "USDC".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Base".to_string(),
                    chain_type: "Base".to_string(),
                    chain_id: Some(8453),
                    contract_address: Some("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".to_string()),
                    decimals: 6,
                    symbol: "USDC".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Arbitrum".to_string(),
                    chain_type: "Arbitrum".to_string(),
                    chain_id: Some(42161),
                    contract_address: Some("0xaf88d065e77c8cC2239327C5EDb3A432268e5831".to_string()),
                    decimals: 6,
                    symbol: "USDC".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Optimism".to_string(),
                    chain_type: "Optimism".to_string(),
                    chain_id: Some(10),
                    contract_address: Some("0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85".to_string()),
                    decimals: 6,
                    symbol: "USDC".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Polygon".to_string(),
                    chain_type: "Polygon".to_string(),
                    chain_id: Some(137),
                    contract_address: Some("0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174".to_string()),
                    decimals: 6,
                    symbol: "USDC".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Avalanche".to_string(),
                    chain_type: "Avalanche".to_string(),
                    chain_id: Some(43114),
                    contract_address: Some("0xB97EF9Ef8734C71904D8002F8b6Bc66Dd9c48a6E".to_string()),
                    decimals: 6,
                    symbol: "USDC".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                // Solana
                ChainDeployment {
                    chain: "Solana".to_string(),
                    chain_type: "Solana".to_string(),
                    chain_id: Some(101),
                    contract_address: Some("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string()),
                    decimals: 6,
                    symbol: "USDC".to_string(),
                    is_native: false,
                    token_standard: "SPL".to_string(),
                },
                // Additional chains
                ChainDeployment {
                    chain: "Tron".to_string(),
                    chain_type: "Tron".to_string(),
                    chain_id: None,
                    contract_address: Some("TEkxiTehnzSmSe2XqrBj4w32RUN966rdz8".to_string()),
                    decimals: 6,
                    symbol: "USDC".to_string(),
                    is_native: false,
                    token_standard: "TRC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Sui".to_string(),
                    chain_type: "Sui".to_string(),
                    chain_id: None,
                    contract_address: Some("0x5d4b302506645c37ff133b98c4b50a5ae14841659738d6d733d59d0d217a93bf::coin::COIN".to_string()),
                    decimals: 6,
                    symbol: "USDC".to_string(),
                    is_native: false,
                    token_standard: "Sui Coin".to_string(),
                },
                ChainDeployment {
                    chain: "Near".to_string(),
                    chain_type: "Near".to_string(),
                    chain_id: None,
                    contract_address: Some("17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1".to_string()),
                    decimals: 6,
                    symbol: "USDC.e".to_string(),
                    is_native: false,
                    token_standard: "NEP-141".to_string(),
                },
                ChainDeployment {
                    chain: "Tezos".to_string(),
                    chain_type: "Tezos".to_string(),
                    chain_id: None,
                    contract_address: Some("KT1UsSfaXyqcjSVPeiD7U1bWgKy3taYN7NWY".to_string()),
                    decimals: 6,
                    symbol: "USDc".to_string(),
                    is_native: false,
                    token_standard: "FA2".to_string(),
                },
                ChainDeployment {
                    chain: "Filecoin".to_string(),
                    chain_type: "Filecoin".to_string(),
                    chain_id: Some(314),
                    contract_address: Some("0x2421db204968A367CC2C866CD057fA754Cb84EdF".to_string()),
                    decimals: 6,
                    symbol: "USDC".to_string(),
                    is_native: false,
                    token_standard: "FRC-20".to_string(),
                },
            ],
        });
    }
    
    /// Initialize wrapped tokens
    fn initialize_wrapped_tokens(&mut self) {
        // WBTC - Wrapped Bitcoin
        self.add_token(UnifiedToken {
            base_symbol: "WBTC".to_string(),
            name: "Wrapped Bitcoin".to_string(),
            cmc_id: Some(3717),
            coingecko_id: Some("wrapped-bitcoin".to_string()),
            category: "Wrapped".to_string(),
            is_multi_chain: true,
            asset_type: AssetType::Wrapped,
            deployments: vec![
                ChainDeployment {
                    chain: "Ethereum".to_string(),
                    chain_type: "Ethereum".to_string(),
                    chain_id: Some(1),
                    contract_address: Some("0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599".to_string()),
                    decimals: 8,
                    symbol: "WBTC".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Base".to_string(),
                    chain_type: "Base".to_string(),
                    chain_id: Some(8453),
                    contract_address: Some("0x1ceA84203673764244E05693e42E6Ace62bE9BA5".to_string()),
                    decimals: 8,
                    symbol: "WBTC".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Arbitrum".to_string(),
                    chain_type: "Arbitrum".to_string(),
                    chain_id: Some(42161),
                    contract_address: Some("0x2f2a2543B76A4166549F7aaB2e75Bef0aefC5B0f".to_string()),
                    decimals: 8,
                    symbol: "WBTC".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Optimism".to_string(),
                    chain_type: "Optimism".to_string(),
                    chain_id: Some(10),
                    contract_address: Some("0x68f180fcCe6836688e9084f035309E29Bf0A2095".to_string()),
                    decimals: 8,
                    symbol: "WBTC".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Polygon".to_string(),
                    chain_type: "Polygon".to_string(),
                    chain_id: Some(137),
                    contract_address: Some("0x1bfd67037b42cf73acF2047067bd4F2C47D9BfD6".to_string()),
                    decimals: 8,
                    symbol: "WBTC".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Solana".to_string(),
                    chain_type: "Solana".to_string(),
                    chain_id: Some(101),
                    contract_address: Some("9n4nbM75f5Ui33ZbPYXn59EwSgE8CGsHtAeTH5YFeJ9E".to_string()),
                    decimals: 6,
                    symbol: "BTC".to_string(),
                    is_native: false,
                    token_standard: "SPL".to_string(),
                },
                ChainDeployment {
                    chain: "Avalanche".to_string(),
                    chain_type: "Avalanche".to_string(),
                    chain_id: Some(43114),
                    contract_address: Some("0x50b7545627a5162F82A992c33b87aDc75187B218".to_string()),
                    decimals: 8,
                    symbol: "WBTC.e".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Sui".to_string(),
                    chain_type: "Sui".to_string(),
                    chain_id: None,
                    contract_address: Some("0x027792d9fed7f9844eb4839566001bb6f6cb4804f66aa2da6fe1ee242d896881::coin::COIN".to_string()),
                    decimals: 8,
                    symbol: "WBTC".to_string(),
                    is_native: false,
                    token_standard: "Sui Coin".to_string(),
                },
                ChainDeployment {
                    chain: "Near".to_string(),
                    chain_type: "Near".to_string(),
                    chain_id: None,
                    contract_address: Some("2260fac5e5542a773aa44fbcfedf7c193bc2c599.factory.bridge.near".to_string()),
                    decimals: 8,
                    symbol: "WBTC".to_string(),
                    is_native: false,
                    token_standard: "NEP-141".to_string(),
                },
                ChainDeployment {
                    chain: "Tezos".to_string(),
                    chain_type: "Tezos".to_string(),
                    chain_id: None,
                    contract_address: Some("KT1PWx2mnDueood7fEmfbBDKx1D9BAnnXitn".to_string()),
                    decimals: 8,
                    symbol: "tzBTC".to_string(),
                    is_native: false,
                    token_standard: "FA1.2".to_string(),
                },
            ],
        });
    }
    
    /// Initialize DeFi tokens (multichain)
    fn initialize_defi_tokens(&mut self) {
        // LINK - Chainlink
        self.add_defi_token(
            "LINK", 
            "Chainlink", 
            1975, 
            "chainlink",
            vec![
                ("Ethereum", "0x514910771AF9Ca656af840dff83E8264EcF986CA", 18),
                ("Base", "0x88Fb150BDc53A65fe94Dea0c9BA0a6dAf8C6e196", 18),
                ("Arbitrum", "0xf97f4df75117a78c1A5a0DBb814Af92458539FB4", 18),
                ("Optimism", "0x350a791Bfc2C21F9Ed5d10980Dad2e2638ffa7f6", 18),
                ("Polygon", "0x53E0bca35eC356BD5ddDFebbD1Fc0fD03FaBad39", 18),
                ("Avalanche", "0x5947BB275c521040051D82396192181b413227A3", 18),
                ("Solana", "ChainLinkXHhLLEKUwRWGfLUsaa1Sba8VuKB5riJVkAP", 6),
            ]
        );
        
        // UNI - Uniswap
        self.add_defi_token(
            "UNI",
            "Uniswap",
            7083,
            "uniswap",
            vec![
                ("Ethereum", "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984", 18),
                ("Base", "0xc3De830EA07524a0761646a6a4e4be0e114a3C83", 18),
                ("Arbitrum", "0xFa7F8980b0f1E64A2062791cc3b0871572f1F7f0", 18),
                ("Optimism", "0x6fd9d7AD17242c41f7131d257212c54A0e816691", 18),
                ("Polygon", "0xb33EaAd8d922B1083446DC23f610c2567fB5180f", 18),
                ("Avalanche", "0x8eBAf22B6F053dFFeaf46f4Dd9eFA95D89ba8580", 18),
                ("Solana", "8FU95xFJhUUkyyCLU13HSzDLs7oC4QZdXQHL6SCeab36", 6),
            ]
        );
        
        // AAVE
        self.add_defi_token(
            "AAVE",
            "Aave",
            7278,
            "aave",
            vec![
                ("Ethereum", "0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9", 18),
                ("Base", "0x18Cd890F4e23422DC4aa8C2D6E0Bd3F3bD8873C8", 18),
                ("Arbitrum", "0xba5DdD1f9d7F570dc94a51479a000E3BCE967196", 18),
                ("Optimism", "0x76FB31fb4af56892A25e32cFC43De717950c9278", 18),
                ("Polygon", "0xD6DF932A45C0f255f85145f286eA0b292B21C90B", 18),
                ("Avalanche", "0x63a72806098Bd3D9520cC43356dD78afe5D386D9", 18),
                ("Solana", "AobVSwdW9BbpMdJvTqeCN8hzbQjpELhfMRQh9Q3rGfYn", 6),
            ]
        );
        
        // Single-chain DeFi tokens
        self.add_single_chain_token("MKR", "MakerDAO", 1518, "maker", "Ethereum", "0x9f8F72aA9304c8B593d555F12eF6589cC3A579A2", 18);
        self.add_single_chain_token("LDO", "Lido DAO", 8000, "lido-dao", "Ethereum", "0x5A98FcBEA516Cf06857215779Fd812CA3beF1B32", 18);
        self.add_single_chain_token("IMX", "Immutable X", 10603, "immutable-x", "Ethereum", "0xF57e7e7C23978C3cAEC3C3548E3D615c346e79fF", 18);
        self.add_single_chain_token("ENA", "Ethena", 30171, "ethena", "Ethereum", "0x57e114B691Db790C35207b2e685D4A43181e6061", 18);
        self.add_single_chain_token("ONDO", "Ondo Finance", 21159, "ondo-finance", "Ethereum", "0xfAbA6f8e4a5E8Ab82F62fe7C39859FA577269BE3", 18);
    }
    
    /// Initialize Layer 2 governance tokens
    fn initialize_layer2_tokens(&mut self) {
        self.add_single_chain_token("ARB", "Arbitrum", 11841, "arbitrum", "Arbitrum", "0x912CE59144191C1204E64559FE8253a0e49E6548", 18);
        self.add_single_chain_token("OP", "Optimism", 11840, "optimism", "Optimism", "0x4200000000000000000000000000000000000042", 18);
        self.add_single_chain_token("MNT", "Mantle", 27075, "mantle", "Ethereum", "0x3c3a81e81dc49A522A592e7622A7E711c06bf354", 18);
    }
    
    /// Initialize Solana ecosystem tokens
    fn initialize_solana_ecosystem(&mut self) {
        // JUP - Jupiter
        self.add_token(UnifiedToken {
            base_symbol: "JUP".to_string(),
            name: "Jupiter".to_string(),
            cmc_id: Some(29210),
            coingecko_id: Some("jupiter-exchange-solana".to_string()),
            category: "DEX Aggregator".to_string(),
            is_multi_chain: true,
            asset_type: AssetType::Token,
            deployments: vec![
                ChainDeployment {
                    chain: "Solana".to_string(),
                    chain_type: "Solana".to_string(),
                    chain_id: Some(101),
                    contract_address: Some("JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN".to_string()),
                    decimals: 6,
                    symbol: "JUP".to_string(),
                    is_native: false,
                    token_standard: "SPL".to_string(),
                },
                ChainDeployment {
                    chain: "Ethereum".to_string(),
                    chain_type: "Ethereum".to_string(),
                    chain_id: Some(1),
                    contract_address: Some("0x4B1E80cAC91e2216EEb63e29B957eB91Ae9C2Be8".to_string()),
                    decimals: 18,
                    symbol: "JUP".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
            ],
        });
        
        // PYTH - Pyth Network
        self.add_token(UnifiedToken {
            base_symbol: "PYTH".to_string(),
            name: "Pyth Network".to_string(),
            cmc_id: Some(28177),
            coingecko_id: Some("pyth-network".to_string()),
            category: "Oracle".to_string(),
            is_multi_chain: true,
            asset_type: AssetType::Token,
            deployments: vec![
                ChainDeployment {
                    chain: "Solana".to_string(),
                    chain_type: "Solana".to_string(),
                    chain_id: Some(101),
                    contract_address: Some("HZ1JovNiVvGrGNiiYvEozEVgZ58xaU3RKwX8eACQBCt3".to_string()),
                    decimals: 6,
                    symbol: "PYTH".to_string(),
                    is_native: false,
                    token_standard: "SPL".to_string(),
                },
                ChainDeployment {
                    chain: "Ethereum".to_string(),
                    chain_type: "Ethereum".to_string(),
                    chain_id: Some(1),
                    contract_address: Some("0x31429d1856aD1377A8b0250511119c17C4e71A64".to_string()),
                    decimals: 18,
                    symbol: "PYTH".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Arbitrum".to_string(),
                    chain_type: "Arbitrum".to_string(),
                    chain_id: Some(42161),
                    contract_address: Some("0xE4D5c6aE46EDFFA3F79Af539A755D83e88dAAF31".to_string()),
                    decimals: 18,
                    symbol: "PYTH".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
                ChainDeployment {
                    chain: "Base".to_string(),
                    chain_type: "Base".to_string(),
                    chain_id: Some(8453),
                    contract_address: Some("0x2880aB155794e7179c9eE2e38200202908C17B43".to_string()),
                    decimals: 18,
                    symbol: "PYTH".to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
            ],
        });
    }
    
    /// Initialize Cosmos ecosystem tokens
    fn initialize_cosmos_ecosystem(&mut self) {
        self.add_native_token("ATOM", "Cosmos", 3794, "cosmos", "Cosmos", 6);
        self.add_native_token("OSMO", "Osmosis", 12220, "osmosis", "Osmosis", 6);
        self.add_native_token("SCRT", "Secret", 5604, "secret", "Secret", 6);
        self.add_native_token("JUNO", "JUNO", 14299, "juno-network", "Juno", 6);
        self.add_native_token("AKT", "Akash Network", 7431, "akash-network", "Akash", 6);
        self.add_native_token("TIA", "Celestia", 22861, "celestia", "Celestia", 6);
        self.add_native_token("SEI", "Sei", 23149, "sei-network", "Sei", 6);
        self.add_native_token("INJ", "Injective", 7226, "injective-protocol", "Injective", 18);
    }
    
    // Helper methods
    fn add_token(&mut self, token: UnifiedToken) {
        self.tokens.insert(token.base_symbol.clone(), token);
    }
    
    fn add_native_token(&mut self, symbol: &str, name: &str, cmc_id: u32, coingecko: &str, chain: &str, decimals: u8) {
        self.add_token(UnifiedToken {
            base_symbol: symbol.to_string(),
            name: name.to_string(),
            cmc_id: Some(cmc_id),
            coingecko_id: Some(coingecko.to_string()),
            category: "Layer 1".to_string(),
            is_multi_chain: false,
            asset_type: AssetType::Native,
            deployments: vec![
                ChainDeployment {
                    chain: chain.to_string(),
                    chain_type: chain.to_string(),
                    chain_id: None,
                    contract_address: None,
                    decimals,
                    symbol: symbol.to_string(),
                    is_native: true,
                    token_standard: "Native".to_string(),
                },
            ],
        });
    }
    
    fn add_single_chain_token(&mut self, symbol: &str, name: &str, cmc_id: u32, coingecko: &str, chain: &str, address: &str, decimals: u8) {
        let chain_type = match chain {
            "Ethereum" => "Ethereum",
            "Base" => "Base",
            "Arbitrum" => "Arbitrum",
            "Optimism" => "Optimism",
            "Polygon" => "Polygon",
            _ => chain,
        };
        
        self.add_token(UnifiedToken {
            base_symbol: symbol.to_string(),
            name: name.to_string(),
            cmc_id: Some(cmc_id),
            coingecko_id: Some(coingecko.to_string()),
            category: "DeFi".to_string(),
            is_multi_chain: false,
            asset_type: AssetType::Token,
            deployments: vec![
                ChainDeployment {
                    chain: chain.to_string(),
                    chain_type: chain_type.to_string(),
                    chain_id: None,
                    contract_address: Some(address.to_string()),
                    decimals,
                    symbol: symbol.to_string(),
                    is_native: false,
                    token_standard: "ERC-20".to_string(),
                },
            ],
        });
    }
    
    fn add_defi_token(&mut self, symbol: &str, name: &str, cmc_id: u32, coingecko: &str, deployments: Vec<(&str, &str, u8)>) {
        let chain_deployments: Vec<ChainDeployment> = deployments.iter().map(|(chain, address, decimals)| {
            ChainDeployment {
                chain: chain.to_string(),
                chain_type: chain.to_string(),
                chain_id: None,
                contract_address: Some(address.to_string()),
                decimals: *decimals,
                symbol: symbol.to_string(),
                is_native: false,
                token_standard: "ERC-20".to_string(),
            }
        }).collect();
        
        self.add_token(UnifiedToken {
            base_symbol: symbol.to_string(),
            name: name.to_string(),
            cmc_id: Some(cmc_id),
            coingecko_id: Some(coingecko.to_string()),
            category: "DeFi".to_string(),
            is_multi_chain: deployments.len() > 1,
            asset_type: AssetType::Token,
            deployments: chain_deployments,
        });
    }
    
    // Query methods
    pub fn get_all_tokens(&self) -> Vec<UnifiedToken> {
        self.tokens.values().cloned().collect()
    }
    
    pub fn get_token(&self, symbol: &str) -> Option<&UnifiedToken> {
        self.tokens.get(symbol)
    }
    
    pub fn get_tokens_by_chain(&self, chain: &str) -> Vec<UnifiedToken> {
        self.tokens.values()
            .filter(|token| token.deployments.iter().any(|d| d.chain == chain || d.chain_type == chain))
            .cloned()
            .collect()
    }
    
    pub fn get_stablecoins(&self) -> Vec<UnifiedToken> {
        self.tokens.values()
            .filter(|token| matches!(token.asset_type, AssetType::Stablecoin))
            .cloned()
            .collect()
    }
    
    pub fn get_native_tokens(&self) -> Vec<UnifiedToken> {
        self.tokens.values()
            .filter(|token| matches!(token.asset_type, AssetType::Native))
            .cloned()
            .collect()
    }
    
    pub fn get_multichain_tokens(&self) -> Vec<UnifiedToken> {
        self.tokens.values()
            .filter(|token| token.is_multi_chain)
            .cloned()
            .collect()
    }
    
    pub fn get_token_variants(&self, base_symbol: &str) -> Vec<String> {
        // For now, return just the base symbol
        // In future could return different versions (e.g., WETH, stETH for ETH)
        vec![base_symbol.to_string()]
    }
    
    pub fn get_token_deployment(&self, base_symbol: &str, chain: &str) -> Option<ChainDeployment> {
        self.get_token(base_symbol)
            .and_then(|token| {
                token.deployments.iter()
                    .find(|d| d.chain == chain || d.chain_type == chain)
                    .cloned()
            })
    }
    
    pub fn is_multi_chain_token(&self, base_symbol: &str) -> bool {
        self.get_token(base_symbol)
            .map(|token| token.is_multi_chain)
            .unwrap_or(false)
    }
}

// Type alias for compatibility
pub type TokenRegistry = CompleteTokenRegistry;

// Global registry instance
static REGISTRY: Lazy<TokenRegistry> = Lazy::new(|| TokenRegistry::new());

// Function to get the global registry
pub fn get_token_registry() -> &'static TokenRegistry {
    &REGISTRY
}