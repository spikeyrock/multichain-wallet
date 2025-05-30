use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use sha2::{Sha512, Digest};

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::ApiResult;

pub struct Hedera;

impl Hedera {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Hedera {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Hedera)
    }

    async fn generate_address(
        &self,
        seed: &[u8],
        _passphrase: &str,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Hedera uses Ed25519
        let path = self.derivation_path(index);
        
        // Derive key using SLIP-0010 for Ed25519
        let derived_key = self.derive_ed25519_key(seed, &path)?;
        
        let signing_key = SigningKey::from_bytes(&derived_key);
        let verifying_key = signing_key.verifying_key();
        
        // Hedera uses account IDs like 0.0.12345, but we'll store the public key
        // In real usage, the account ID is assigned when the account is created on-chain
        // For now, we'll use a placeholder format with the public key
        let address = format!("0.0.{}", hex::encode(&verifying_key.as_bytes()[..8]));
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Hedera,
            chain_info: self.info(),
            derivation_path: path.to_string_all_hardened(),
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // Hedera uses m/44'/3030'/0'/0'/index'
        DerivationPath::new(44, 3030, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // Hedera addresses are in format 0.0.xxxxx where xxxxx is the account number
        let parts: Vec<&str> = address.split('.').collect();
        
        if parts.len() != 3 {
            return false;
        }
        
        // First two parts should be "0"
        if parts[0] != "0" || parts[1] != "0" {
            return false;
        }
        
        // Third part should be a valid number or hex string
        !parts[2].is_empty()
    }

    fn example_address(&self) -> &str {
        "0.0.123456"
    }
}

impl Hedera {
    // SLIP-0010 Ed25519 derivation
    fn derive_ed25519_key(&self, seed: &[u8], path: &DerivationPath) -> ApiResult<[u8; 32]> {
        // Start with the master key
        let mut hasher = Sha512::new();
        hasher.update(b"ed25519 seed");
        hasher.update(seed);
        let mut key = hasher.finalize();
        
        // Derive through the path - Hedera uses all hardened derivation
        let indices = vec![
            0x80000000 + path.purpose,    // 44'
            0x80000000 + path.coin_type,  // 3030'
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