use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChainInfo {
    pub name: String,
    pub symbol: String,
    pub coin_type: u32,  // BIP44 coin type
    pub decimals: u8,
    pub address_format: AddressFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AddressFormat {
    Bitcoin { prefix: String },
    Ethereum,
    Base58 { version: u8 },
    Bech32 { hrp: String },
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ChainType {
    BitcoinLegacy,
    BitcoinSegwit,
    BitcoinTaproot,
    Ethereum,
    Ripple,
    Solana,
    Tron,
    Cardano,
    Sui,
    Stellar,
    Monero,
    Near,
}

impl fmt::Display for ChainType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let info = get_chain_info(self);
        write!(f, "{} ({})", info.name, info.symbol)
    }
}

// Chain registry - add new chains here
pub fn get_chain_info(chain_type: &ChainType) -> ChainInfo {
    match chain_type {
        ChainType::BitcoinLegacy => ChainInfo {
            name: "Bitcoin".to_string(),
            symbol: "BTC".to_string(),
            coin_type: 0,
            decimals: 8,
            address_format: AddressFormat::Bitcoin { 
                prefix: "1".to_string() 
            },
        },
        ChainType::BitcoinSegwit => ChainInfo {
            name: "Bitcoin".to_string(),
            symbol: "BTC".to_string(),
            coin_type: 0,
            decimals: 8,
            address_format: AddressFormat::Bech32 { 
                hrp: "bc".to_string() 
            },
        },
        ChainType::BitcoinTaproot => ChainInfo {
            name: "Bitcoin".to_string(),
            symbol: "BTC".to_string(),
            coin_type: 0,
            decimals: 8,
            address_format: AddressFormat::Bech32 { 
                hrp: "bc".to_string() 
            },
        },
        ChainType::Ethereum => ChainInfo {
            name: "Ethereum".to_string(),
            symbol: "ETH".to_string(),
            coin_type: 60,
            decimals: 18,
            address_format: AddressFormat::Ethereum,
        },
        ChainType::Ripple => ChainInfo {
            name: "Ripple".to_string(),
            symbol: "XRP".to_string(),
            coin_type: 144,
            decimals: 6,
            address_format: AddressFormat::Base58 { version: 0 },
        },
        ChainType::Solana => ChainInfo {
            name: "Solana".to_string(),
            symbol: "SOL".to_string(),
            coin_type: 501,
            decimals: 9,
            address_format: AddressFormat::Base58 { version: 0 },
        },
        ChainType::Tron => ChainInfo {
            name: "TRON".to_string(),
            symbol: "TRX".to_string(),
            coin_type: 195,
            decimals: 6,
            address_format: AddressFormat::Base58 { version: 0x41 },
        },
        ChainType::Cardano => ChainInfo {
            name: "Cardano".to_string(),
            symbol: "ADA".to_string(),
            coin_type: 1815,
            decimals: 6,
            address_format: AddressFormat::Bech32 { 
                hrp: "addr".to_string() 
            },
        },
        ChainType::Sui => ChainInfo {
            name: "Sui".to_string(),
            symbol: "SUI".to_string(),
            coin_type: 784,
            decimals: 9,
            address_format: AddressFormat::Custom("0x".to_string()),
        },
        ChainType::Stellar => ChainInfo {
            name: "Stellar".to_string(),
            symbol: "XLM".to_string(),
            coin_type: 148,
            decimals: 7,
            address_format: AddressFormat::Custom("G".to_string()),
        },
        ChainType::Monero => ChainInfo {
            name: "Monero".to_string(),
            symbol: "XMR".to_string(),
            coin_type: 128,
            decimals: 12,
            address_format: AddressFormat::Custom("4".to_string()),
        },
        ChainType::Near => ChainInfo {
            name: "NEAR Protocol".to_string(),
            symbol: "NEAR".to_string(),
            coin_type: 397,
            decimals: 24,
            address_format: AddressFormat::Custom("implicit".to_string()),
        },
    }
}

// Get all supported chains
pub fn get_all_chain_types() -> Vec<ChainType> {
    vec![
        ChainType::BitcoinLegacy,
        ChainType::BitcoinSegwit,
        ChainType::BitcoinTaproot,
        ChainType::Ethereum,
        ChainType::Ripple,
        ChainType::Solana,
        ChainType::Tron,
        ChainType::Cardano,
        ChainType::Sui,
        ChainType::Stellar,
        ChainType::Monero,
        ChainType::Near,
    ]
}