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
    Sui,
    Near,
    Dogecoin,
    Cosmos,
    Osmosis,
    Juno,
    Secret,
    Akash,
    Sei,
    Celestia,
    Injective,
    Tezos,
    Filecoin,
    // Layer 2 EVM chains
    Base,
    Arbitrum,
    Optimism,
    Polygon,
    Avalanche,
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
        ChainType::Sui => ChainInfo {
            name: "Sui".to_string(),
            symbol: "SUI".to_string(),
            coin_type: 784,
            decimals: 9,
            address_format: AddressFormat::Custom("0x".to_string()),
        },
        ChainType::Near => ChainInfo {
            name: "NEAR Protocol".to_string(),
            symbol: "NEAR".to_string(),
            coin_type: 397,
            decimals: 24,
            address_format: AddressFormat::Custom("implicit".to_string()),
        },
        ChainType::Dogecoin => ChainInfo {
            name: "Dogecoin".to_string(),
            symbol: "DOGE".to_string(),
            coin_type: 3,
            decimals: 8,
            address_format: AddressFormat::Bitcoin { 
                prefix: "D".to_string() 
            },
        },
        ChainType::Cosmos => ChainInfo {
            name: "Cosmos".to_string(),
            symbol: "ATOM".to_string(),
            coin_type: 118,
            decimals: 6,
            address_format: AddressFormat::Bech32 { 
                hrp: "cosmos".to_string() 
            },
        },
        ChainType::Osmosis => ChainInfo {
            name: "Osmosis".to_string(),
            symbol: "OSMO".to_string(),
            coin_type: 118,
            decimals: 6,
            address_format: AddressFormat::Bech32 { 
                hrp: "osmo".to_string() 
            },
        },
        ChainType::Juno => ChainInfo {
            name: "Juno".to_string(),
            symbol: "JUNO".to_string(),
            coin_type: 118,
            decimals: 6,
            address_format: AddressFormat::Bech32 { 
                hrp: "juno".to_string() 
            },
        },
        ChainType::Secret => ChainInfo {
            name: "Secret Network".to_string(),
            symbol: "SCRT".to_string(),
            coin_type: 529,
            decimals: 6,
            address_format: AddressFormat::Bech32 { 
                hrp: "secret".to_string() 
            },
        },
        ChainType::Akash => ChainInfo {
            name: "Akash".to_string(),
            symbol: "AKT".to_string(),
            coin_type: 118,
            decimals: 6,
            address_format: AddressFormat::Bech32 { 
                hrp: "akash".to_string() 
            },
        },
        ChainType::Sei => ChainInfo {
            name: "Sei".to_string(),
            symbol: "SEI".to_string(),
            coin_type: 118,
            decimals: 6,
            address_format: AddressFormat::Bech32 { 
                hrp: "sei".to_string() 
            },
        },
        ChainType::Celestia => ChainInfo {
            name: "Celestia".to_string(),
            symbol: "TIA".to_string(),
            coin_type: 118,
            decimals: 6,
            address_format: AddressFormat::Bech32 { 
                hrp: "celestia".to_string() 
            },
        },
        ChainType::Injective => ChainInfo {
            name: "Injective".to_string(),
            symbol: "INJ".to_string(),
            coin_type: 60,
            decimals: 18,
            address_format: AddressFormat::Bech32 { 
                hrp: "inj".to_string() 
            },
        },
        ChainType::Tezos => ChainInfo {
            name: "Tezos".to_string(),
            symbol: "XTZ".to_string(),
            coin_type: 1729,
            decimals: 6,
            address_format: AddressFormat::Custom("tz1".to_string()),
        },
        ChainType::Filecoin => ChainInfo {
            name: "Filecoin".to_string(),
            symbol: "FIL".to_string(),
            coin_type: 461,
            decimals: 18,
            address_format: AddressFormat::Custom("f1".to_string()),
        },
        // Layer 2 EVM chains
        ChainType::Base => ChainInfo {
            name: "Base".to_string(),
            symbol: "ETH".to_string(),
            coin_type: 60, // Same as Ethereum
            decimals: 18,
            address_format: AddressFormat::Ethereum,
        },
        ChainType::Arbitrum => ChainInfo {
            name: "Arbitrum".to_string(),
            symbol: "ETH".to_string(),
            coin_type: 60, // Same as Ethereum
            decimals: 18,
            address_format: AddressFormat::Ethereum,
        },
        ChainType::Optimism => ChainInfo {
            name: "Optimism".to_string(),
            symbol: "ETH".to_string(),
            coin_type: 60, // Same as Ethereum
            decimals: 18,
            address_format: AddressFormat::Ethereum,
        },
        ChainType::Polygon => ChainInfo {
            name: "Polygon".to_string(),
            symbol: "MATIC".to_string(),
            coin_type: 60, // Same as Ethereum
            decimals: 18,
            address_format: AddressFormat::Ethereum,
        },
        ChainType::Avalanche => ChainInfo {
            name: "Avalanche".to_string(),
            symbol: "AVAX".to_string(),
            coin_type: 60, // Same as Ethereum for C-Chain
            decimals: 18,
            address_format: AddressFormat::Ethereum,
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
        ChainType::Sui,
        ChainType::Near,
        ChainType::Dogecoin,
        ChainType::Cosmos,
        ChainType::Osmosis,
        ChainType::Juno,
        ChainType::Secret,
        ChainType::Akash,
        ChainType::Sei,
        ChainType::Celestia,
        ChainType::Injective,
        ChainType::Tezos,
        ChainType::Filecoin,
        // Layer 2s
        ChainType::Base,
        ChainType::Arbitrum,
        ChainType::Optimism,
        ChainType::Polygon,
        ChainType::Avalanche,
    ]
}

// Get chain types by symbol
pub fn get_chain_types_by_symbol(symbol: &str) -> Vec<ChainType> {
    match symbol.to_uppercase().as_str() {
        "BTC" => vec![
            ChainType::BitcoinLegacy,
            ChainType::BitcoinSegwit,
            ChainType::BitcoinTaproot,
        ],
        "ETH" => vec![
            ChainType::Ethereum,
            ChainType::Base,
            ChainType::Arbitrum,
            ChainType::Optimism,
        ],
        "XRP" => vec![ChainType::Ripple],
        "SOL" => vec![ChainType::Solana],
        "TRX" => vec![ChainType::Tron],
        "SUI" => vec![ChainType::Sui],
        "NEAR" => vec![ChainType::Near],
        "DOGE" => vec![ChainType::Dogecoin],
        "ATOM" => vec![ChainType::Cosmos],
        "OSMO" => vec![ChainType::Osmosis],
        "JUNO" => vec![ChainType::Juno],
        "SCRT" => vec![ChainType::Secret],
        "AKT" => vec![ChainType::Akash],
        "SEI" => vec![ChainType::Sei],
        "TIA" => vec![ChainType::Celestia],
        "INJ" => vec![ChainType::Injective],
        "XTZ" => vec![ChainType::Tezos],
        "FIL" => vec![ChainType::Filecoin],
        "MATIC" => vec![ChainType::Polygon],
        "AVAX" => vec![ChainType::Avalanche],
        _ => vec![],
    }
}