use serde::{Deserialize, Serialize};
use crate::core::ChainType;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateMnemonicRequest {
    pub language: String,
    pub word_count: u32,
}

impl GenerateMnemonicRequest {
    pub fn validate(&self) -> Result<(), String> {
        // Validate language
        let valid_languages = vec![
            "english", "japanese", "korean", "spanish", 
            "chinese_simplified", "chinese_traditional", 
            "french", "italian", "czech", "portuguese"
        ];
        
        if !valid_languages.contains(&self.language.to_lowercase().as_str()) {
            return Err(format!("Invalid language: {}", self.language));
        }

        // Validate word count
        match self.word_count {
            12 | 15 | 18 | 21 | 24 => Ok(()),
            _ => Err(format!("Invalid word count: {}. Must be 12, 15, 18, 21, or 24", self.word_count))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateMnemonicResponse {
    pub mnemonic: String,
    pub language: String,
    pub word_count: u32,
    pub generated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateMnemonicRequest {
    pub mnemonic: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateMnemonicResponse {
    pub valid: bool,
    pub word_count: Option<u32>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupportedLanguagesResponse {
    pub languages: Vec<LanguageInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageInfo {
    pub code: String,
    pub name: String,
    pub native_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateWalletRequest {
    pub mnemonic: String,
    #[serde(default)]
    pub passphrase: String,
    #[serde(default)]
    pub index: u32,
    pub symbol: String,  // Changed from address_type to symbol
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AddressType {
    BitcoinTaproot,
    BitcoinSegwit,
    BitcoinLegacy,
    Ethereum,
    Xrp,
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

}

// Convert from AddressType to ChainType
impl From<AddressType> for ChainType {
    fn from(address_type: AddressType) -> Self {
        match address_type {
            AddressType::BitcoinTaproot => ChainType::BitcoinTaproot,
            AddressType::BitcoinSegwit => ChainType::BitcoinSegwit,
            AddressType::BitcoinLegacy => ChainType::BitcoinLegacy,
            AddressType::Ethereum => ChainType::Ethereum,
            AddressType::Xrp => ChainType::Ripple,
            AddressType::Solana => ChainType::Solana,
            AddressType::Tron => ChainType::Tron,
            AddressType::Sui => ChainType::Sui,
            AddressType::Near => ChainType::Near,
            AddressType::Dogecoin => ChainType::Dogecoin,
            AddressType::Cosmos => ChainType::Cosmos,
            AddressType::Osmosis => ChainType::Osmosis,
            AddressType::Juno => ChainType::Juno,
            AddressType::Secret => ChainType::Secret,
            AddressType::Akash => ChainType::Akash,
            AddressType::Sei => ChainType::Sei,
            AddressType::Celestia => ChainType::Celestia,
            AddressType::Injective => ChainType::Injective,
            AddressType::Tezos => ChainType::Tezos,
            AddressType::Filecoin => ChainType::Filecoin,
        }
    }
}

// Convert from ChainType to AddressType
impl From<ChainType> for AddressType {
    fn from(chain_type: ChainType) -> Self {
        match chain_type {
            ChainType::BitcoinTaproot => AddressType::BitcoinTaproot,
            ChainType::BitcoinSegwit => AddressType::BitcoinSegwit,
            ChainType::BitcoinLegacy => AddressType::BitcoinLegacy,
            ChainType::Ethereum => AddressType::Ethereum,
            ChainType::Ripple => AddressType::Xrp,
            ChainType::Solana => AddressType::Solana,
            ChainType::Tron => AddressType::Tron,
            ChainType::Sui => AddressType::Sui,
            ChainType::Near => AddressType::Near,
            ChainType::Dogecoin => AddressType::Dogecoin,
            ChainType::Cosmos => AddressType::Cosmos,
            ChainType::Osmosis => AddressType::Osmosis,
            ChainType::Juno => AddressType::Juno,
            ChainType::Secret => AddressType::Secret,
            ChainType::Akash => AddressType::Akash,
            ChainType::Sei => AddressType::Sei,
            ChainType::Celestia => AddressType::Celestia,
            ChainType::Injective => AddressType::Injective,
            ChainType::Tezos => AddressType::Tezos,
            ChainType::Filecoin => AddressType::Filecoin,
            // Layer 2 EVM chains use Ethereum address type
            ChainType::Base => AddressType::Ethereum,
            ChainType::Arbitrum => AddressType::Ethereum,
            ChainType::Optimism => AddressType::Ethereum,
            ChainType::Polygon => AddressType::Ethereum,
            ChainType::Avalanche => AddressType::Ethereum,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateWalletResponse {
    pub address: String,
    pub chain_name: String,
    pub chain_symbol: String,
    pub address_type: AddressType,
    pub derivation_path: String,
    pub index: u32,
    pub public_key: String,
    pub private_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_tokens: Option<Vec<TokenInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchGenerateWalletRequest {
    pub mnemonic: String,
    #[serde(default)]
    pub passphrase: String,
    #[serde(default)]
    pub start_index: u32,
    #[serde(default = "default_count")]
    pub count: u32,
    pub symbols: Vec<String>,  // Changed from address_types to symbols
}

fn default_count() -> u32 {
    10
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchGenerateWalletResponse {
    pub addresses: Vec<WalletAddressResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletAddressResponse {
    pub address: String,
    pub chain_name: String,
    pub chain_symbol: String,
    pub address_type: AddressType,
    pub derivation_path: String,
    pub index: u32,
    pub public_key: String,
    pub private_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_tokens: Option<Vec<TokenInfo>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenInfo {
    pub symbol: String,
    pub name: String,
    pub contract_address: Option<String>,
    pub decimals: u8,
    pub token_standard: String,
}

// Backward compatibility
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletAddress {
    pub address: String,
    pub address_type: AddressType,
    pub derivation_path: String,
    pub index: u32,
    pub public_key: String,
    pub private_key: String,
}