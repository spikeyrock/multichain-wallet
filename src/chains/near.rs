use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use sha2::{Sha512, Digest};

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::ApiResult;

pub struct Near;

impl Near {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Near {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Near)
    }

    async fn generate_address(
        &self,
        seed: &[u8],
        _passphrase: &str,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // NEAR uses Ed25519 with SLIP-0010 derivation
        let path = self.derivation_path(index);
        
        // Derive key using SLIP-0010 for Ed25519
        let derived_key = self.derive_ed25519_key(seed, &path)?;
        
        let signing_key = SigningKey::from_bytes(&derived_key);
        let verifying_key = signing_key.verifying_key();
        
        // NEAR implicit address is the hex-encoded public key
        let address = hex::encode(verifying_key.as_bytes());
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Near,
            chain_info: self.info(),
            derivation_path: path.to_string_all_hardened(),
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // NEAR uses m/44'/397'/0'/0'/index'
        DerivationPath::new(44, 397, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // NEAR implicit addresses are 64 character hex strings (32 bytes)
        if address.len() != 64 {
            return false;
        }
        
        address.chars().all(|c| c.is_ascii_hexdigit())
    }

    fn example_address(&self) -> &str {
        "5b3f5f86a7c2dfb8e194f7e8c2e5d8f3a89f8e7d3e4f8a9c7d8e3f5a2c1b4d7e"
    }
}

impl Near {
    // SLIP-0010 Ed25519 derivation
    fn derive_ed25519_key(&self, seed: &[u8], path: &DerivationPath) -> ApiResult<[u8; 32]> {
        // Start with the master key
        let mut hasher = Sha512::new();
        hasher.update(b"ed25519 seed");
        hasher.update(seed);
        let mut key = hasher.finalize();
        
        // Derive through the path - NEAR uses all hardened derivation
        let indices = vec![
            0x80000000 + path.purpose,    // 44'
            0x80000000 + path.coin_type,  // 397'
            0x80000000 + path.account,    // 0'
            0x80000000 + path.change,     // 0'
            0x80000000 + path.index,      // index'
        ];
        
        for index in indices {
            let mut hasher = Sha512::new();
            hasher.update(&[0x00]); // hardened derivation
            hasher.update(&key[..32]); // private key part
            hasher.update(&index.to_be_bytes());
            key = hasher.finalize();
        }
        
        // Return the first 32 bytes as the private key
        let mut private_key = [0u8; 32];
        private_key.copy_from_slice(&key[..32]);
        Ok(private_key)
    }
}