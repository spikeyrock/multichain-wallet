use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use sha2::{Sha512, Digest};

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::ApiResult;

pub struct Solana;

impl Solana {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Solana {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Solana)
    }

    async fn generate_address(
        &self,
        seed: &[u8],
        _passphrase: &str,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Solana uses Ed25519 with SLIP-0010 derivation
        let path = self.derivation_path(index);
        
        // Derive key using SLIP-0010 for Ed25519
        let derived_key = self.derive_ed25519_key(seed, &path)?;
        
        let signing_key = SigningKey::from_bytes(&derived_key);
        let verifying_key = signing_key.verifying_key();
        
        // Solana address is the base58 encoded public key
        let address = bs58::encode(verifying_key.as_bytes()).into_string();
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Solana,
            chain_info: self.info(),
            derivation_path: path.to_string_all_hardened(), // Solana uses hardened derivation
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // Solana uses m/44'/501'/index'/0'
        DerivationPath::new(44, 501, index, 0, 0)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // Validate Solana address (base58 encoded 32-byte public key)
        match bs58::decode(address).into_vec() {
            Ok(bytes) => bytes.len() == 32,
            Err(_) => false,
        }
    }

    fn example_address(&self) -> &str {
        "7VfEGm5pVbPH8oUCZ5aJ8JmaUtxWWd4yqXckLsYBMqpN"
    }
}

impl Solana {
    // SLIP-0010 Ed25519 derivation
    fn derive_ed25519_key(&self, seed: &[u8], path: &DerivationPath) -> ApiResult<[u8; 32]> {
        // Start with the master key
        let mut hasher = Sha512::new();
        hasher.update(b"ed25519 seed");
        hasher.update(seed);
        let mut key = hasher.finalize();
        
        // Derive through the path
        let indices = vec![
            0x80000000 + path.purpose,    // 44'
            0x80000000 + path.coin_type,  // 501'
            0x80000000 + path.account,    // index'
            0x80000000 + path.change,     // 0'
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