use serde::{Deserialize, Serialize};

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

// New wallet generation models

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateWalletRequest {
    pub mnemonic: String,
    #[serde(default)]
    pub passphrase: String,
    #[serde(default)]
    pub index: u32,
    pub address_type: AddressType,
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
    Cardano,
    Sui,
    Stellar,
    Monero,
    Near,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateWalletResponse {
    pub address: String,
    pub address_type: AddressType,
    pub derivation_path: String,
    pub index: u32,
    pub public_key: String,
    pub private_key: String,
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
    pub address_types: Vec<AddressType>,
}

fn default_count() -> u32 {
    10
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchGenerateWalletResponse {
    pub addresses: Vec<WalletAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletAddress {
    pub address: String,
    pub address_type: AddressType,
    pub derivation_path: String,
    pub index: u32,
    pub public_key: String,
    pub private_key: String,
}