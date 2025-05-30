use async_trait::async_trait;
use sha2::{Sha256, Digest};

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::ApiResult;

pub struct Mina;

impl Mina {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Mina {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Mina)
    }

    async fn generate_address(
        &self,
        seed: &[u8],
        _passphrase: &str,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Mina uses a different curve (Pasta curves) which we'll approximate
        // This is a simplified implementation
        let path = self.derivation_path(index);
        
        // Derive a key using SHA256 (simplified)
        let mut hasher = Sha256::new();
        hasher.update(seed);
        hasher.update(&path.purpose.to_le_bytes());
        hasher.update(&path.coin_type.to_le_bytes());
        hasher.update(&path.account.to_le_bytes());
        hasher.update(&path.change.to_le_bytes());
        hasher.update(&path.index.to_le_bytes());
        let private_key = hasher.finalize();
        
        // Generate a simplified public key
        let mut pub_hasher = Sha256::new();
        pub_hasher.update(&private_key);
        pub_hasher.update(b"mina-public");
        let public_key = pub_hasher.finalize();
        
        // Mina uses B62 encoding - a custom base58 variant
        // For simplicity, we'll use standard base58 with "B62" prefix
        let address = format!("B62{}", bs58::encode(&public_key[..20]).into_string());
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Mina,
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(&public_key),
            private_key: hex::encode(&private_key),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // Mina uses m/44'/12586'/0'/0/index
        DerivationPath::new(44, 12586, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // Mina addresses start with B62 and are ~55 characters
        if !address.starts_with("B62") || address.len() < 50 || address.len() > 60 {
            return false;
        }
        
        // Validate base58-like characters
        let addr_part = &address[3..];
        addr_part.chars().all(|c| {
            matches!(c, '1'..='9' | 'A'..='H' | 'J'..='N' | 'P'..='Z' | 'a'..='k' | 'm'..='z')
        })
    }

    fn example_address(&self) -> &str {
        "B62qkYa1o6Mj6uTTjDQCob7FYZspuhkm4RRQhgJg9j4koEBWiSrTQrS"
    }
}