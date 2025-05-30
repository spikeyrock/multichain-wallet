use bip39::{Language, Mnemonic};
use rand::RngCore;
use std::collections::HashMap;
use std::sync::Arc;

use crate::chains;
use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, get_all_chain_types};
use crate::errors::{ApiError, ApiResult};

pub struct WalletService {
    // Cache chains for performance
    chains_cache: HashMap<ChainType, Arc<dyn Chain>>,
}

impl WalletService {
    pub fn new() -> Self {
        Self {
            chains_cache: HashMap::new(),
        }
    }

    fn get_or_create_chain(&mut self, chain_type: &ChainType) -> Arc<dyn Chain> {
        self.chains_cache
            .entry(chain_type.clone())
            .or_insert_with(|| chains::create_chain(chain_type))
            .clone()
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

    pub async fn generate_wallet_address(
        &mut self,
        mnemonic_str: &str,
        passphrase: &str,
        chain_type: &ChainType,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Parse mnemonic
        let mnemonic = Mnemonic::parse(mnemonic_str)
            .map_err(|_| ApiError::InvalidMnemonic)?;

        // Generate seed from mnemonic and passphrase
        let seed = mnemonic.to_seed(passphrase);
        
        // Get or create chain instance
        let chain = self.get_or_create_chain(chain_type);
        
        // Generate address
        chain.generate_address(&seed, passphrase, index).await
    }

    pub async fn batch_generate_wallet_addresses(
        &mut self,
        mnemonic_str: &str,
        passphrase: &str,
        chain_types: &[ChainType],
        start_index: u32,
        count: u32,
    ) -> ApiResult<Vec<WalletAddress>> {
        let mut addresses = Vec::new();
        
        for chain_type in chain_types {
            for i in start_index..(start_index + count) {
                let wallet = self.generate_wallet_address(
                    mnemonic_str,
                    passphrase,
                    chain_type,
                    i,
                ).await?;
                addresses.push(wallet);
            }
        }
        
        Ok(addresses)
    }

    pub async fn get_chain_info(&self, chain_type: &ChainType) -> ChainInfo {
        crate::core::get_chain_info(chain_type)
    }

    pub async fn list_supported_chains(&self) -> Vec<ChainInfo> {
        get_all_chain_types()
            .into_iter()
            .map(|ct| crate::core::get_chain_info(&ct))
            .collect()
    }

    pub async fn validate_address(&mut self, chain_type: &ChainType, address: &str) -> bool {
        let chain = self.get_or_create_chain(chain_type);
        chain.validate_address(address).await
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