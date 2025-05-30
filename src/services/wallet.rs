use bip39::{Language, Mnemonic};
use rand::RngCore;

use crate::errors::{ApiError, ApiResult};

pub struct WalletService {}

impl WalletService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn generate_mnemonic(
        &self,
        language_str: &str,
        word_count: u32,
    ) -> ApiResult<String> {
        let language = self.parse_language(language_str)?;
        
        // Generate entropy based on word count
        let entropy_bytes = match word_count {
            12 => 16, // 128 bits
            15 => 20, // 160 bits
            18 => 24, // 192 bits
            21 => 28, // 224 bits
            24 => 32, // 256 bits
            _ => return Err(ApiError::InvalidWordCount(word_count)),
        };

        // Generate random entropy
        let mut entropy = vec![0u8; entropy_bytes];
        rand::thread_rng().fill_bytes(&mut entropy);

        // Create mnemonic from entropy
        let mnemonic = Mnemonic::from_entropy_in(language, &entropy)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;

        Ok(mnemonic.to_string())
    }

    pub async fn validate_mnemonic(&self, mnemonic_str: &str, language_str: &str) -> (bool, u32) {
        let language = match self.parse_language(language_str) {
            Ok(lang) => lang,
            Err(_) => return (false, 0),
        };

        match Mnemonic::parse_in_normalized(language, mnemonic_str) {
            Ok(mnemonic) => {
                let word_count = mnemonic.word_count() as u32;
                (true, word_count)
            }
            Err(_) => (false, 0),
        }
    }

    fn parse_language(&self, language: &str) -> ApiResult<Language> {
        match language.to_lowercase().as_str() {
            "english" => Ok(Language::English),
            "japanese" => Ok(Language::Japanese),
            "korean" => Ok(Language::Korean),
            "spanish" => Ok(Language::Spanish),
            "chinese_simplified" => Ok(Language::SimplifiedChinese),
            "chinese_traditional" => Ok(Language::TraditionalChinese),
            "french" => Ok(Language::French),
            "italian" => Ok(Language::Italian),
            "czech" => Ok(Language::Czech),
            "portuguese" => Ok(Language::Portuguese),
            _ => Err(ApiError::InvalidLanguage(language.to_string())),
        }
    }
}