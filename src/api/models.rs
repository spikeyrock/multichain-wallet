use serde::{Deserialize, Serialize};

// Health check
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub timestamp: i64,
}

// Mnemonic generation
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateMnemonicRequest {
    pub word_count: u32,
    pub language: String,
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

// Mnemonic validation
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

// Language support
#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageInfo {
    pub code: String,
    pub name: String,
    pub native_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupportedLanguagesResponse {
    pub languages: Vec<LanguageInfo>,
}

// Wallet generation
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateWalletRequest {
    pub mnemonic: String,
    #[serde(default)]
    pub passphrase: String,
    pub symbol: String,
    #[serde(default)]
    pub index: u32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateWalletResponse {
    pub address: String,
    pub chain_name: String,
    pub chain_symbol: String,
    pub address_type: String,
    pub derivation_path: String,
    pub index: u32,
    pub public_key: String,
    pub private_key: String,
    pub supported_tokens: Option<Vec<TokenInfo>>,
}

// Batch wallet generation
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchGenerateWalletRequest {
    pub mnemonic: String,
    #[serde(default)]
    pub passphrase: String,
    pub symbols: Vec<String>,
    #[serde(default)]
    pub start_index: u32,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletAddressResponse {
    pub address: String,
    pub chain_name: String,
    pub chain_symbol: String,
    pub address_type: String,
    pub derivation_path: String,
    pub index: u32,
    pub public_key: String,
    pub private_key: String,
    pub supported_tokens: Option<Vec<TokenInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchGenerateWalletResponse {
    pub addresses: Vec<WalletAddressResponse>,
}

// Token information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenInfo {
    pub symbol: String,
    pub name: String,
    pub contract_address: Option<String>,
    pub decimals: u8,
    pub token_standard: String,
    pub is_native: bool,
}