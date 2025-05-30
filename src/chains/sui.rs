use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use sha2::{Sha512, Digest};
use blake2::Blake2s256;
use blake2::digest::Digest as Blake2Digest;

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::ApiResult;

pub struct Sui;

impl Sui {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Sui {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Sui)
    }

    async fn generate_address(
        &self,
        seed: &[u8],
        _passphrase: &str,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Sui uses Ed25519
        let path = self.derivation_path(index);
        
        // Derive key using SLIP-0010 for Ed25519
        let derived_key = self.derive_ed25519_key(seed, &path)?;
        
        let signing_key = SigningKey::from_bytes(&derived_key);
        let verifying_key = signing_key.verifying_key();
        
        // Sui address is derived from public key
        let mut hasher = Blake2s256::new();
        hasher.update(&[0x00]); // Signature scheme flag for Ed25519
        hasher.update(verifying_key.as_bytes());
        let hash = hasher.finalize();
        
        // Take first 32 bytes and format as 0x prefixed hex
        let address = format!("0x{}", hex::encode(&hash[..32]));
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Sui,
            chain_info: self.info(),
            derivation_path: path.to_string_all_hardened(),
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // Sui uses m/44'/784'/0'/0'/index'
        DerivationPath::new(44, 784, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // Sui addresses are 0x prefixed 64 character hex strings (32 bytes)
        if !address.starts_with("0x") || address.len() != 66 {
            return false;
        }
        
        address[2..].chars().all(|c| c.is_ascii_hexdigit())
    }

    fn example_address(&self) -> &str {
        "0x02a212de6a9dfa3a69e22387acfbafbb1a9e591bd9d636e7895dcfc8de05f331"
    }
}

impl Sui {
    // SLIP-0010 Ed25519 derivation
    fn derive_ed25519_key(&self, seed: &[u8], path: &DerivationPath) -> ApiResult<[u8; 32]> {
        // Start with the master key
        let mut hasher = Sha512::new();
        hasher.update(b"ed25519 seed");
        hasher.update(seed);
        let mut key = hasher.finalize();
        
        // Derive through the path - Sui uses all hardened derivation
        let indices = vec![
            0x80000000 + path.purpose,    // 44'
            0x80000000 + path.coin_type,  // 784'
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