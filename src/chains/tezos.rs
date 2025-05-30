use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use sha2::{Sha512, Digest};
use blake2::{Blake2b, Digest as Blake2Digest};

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::ApiResult;

pub struct Tezos;

impl Tezos {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Tezos {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Tezos)
    }

    async fn generate_address(
        &self,
        seed: &[u8],
        _passphrase: &str,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Tezos uses Ed25519
        let path = self.derivation_path(index);
        
        // Derive key using SLIP-0010 for Ed25519
        let derived_key = self.derive_ed25519_key(seed, &path)?;
        
        let signing_key = SigningKey::from_bytes(&derived_key);
        let verifying_key = signing_key.verifying_key();
        
        // Tezos tz1 address (Ed25519)
        // Prefix: tz1 = [6, 161, 159]
        let mut payload = vec![6, 161, 159];
        
        // Hash the public key with Blake2b
        let mut hasher = Blake2b::<typenum::U32>::new();
        hasher.update(verifying_key.as_bytes());
        let hash = hasher.finalize();
        payload.extend_from_slice(&hash[..20]); // Take first 20 bytes
        
        // Double SHA256 for checksum
        let checksum = sha2::Sha256::digest(&sha2::Sha256::digest(&payload));
        payload.extend_from_slice(&checksum[..4]);
        
        // Base58 encode
        let address = bs58::encode(payload).into_string();
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Tezos,
            chain_info: self.info(),
            derivation_path: path.to_string_all_hardened(),
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // Tezos uses m/44'/1729'/0'/0'/index'
        DerivationPath::new(44, 1729, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // Tezos addresses can start with tz1, tz2, or tz3
        // tz1 is for Ed25519
        if !address.starts_with("tz1") || address.len() != 36 {
            return false;
        }
        
        // Validate base58 and checksum
        match bs58::decode(address).into_vec() {
            Ok(data) => {
                if data.len() != 27 {
                    return false;
                }
                
                let payload = &data[..23];
                let checksum = &data[23..];
                
                let computed_checksum = sha2::Sha256::digest(&sha2::Sha256::digest(payload));
                checksum == &computed_checksum[..4]
            }
            Err(_) => false,
        }
    }

    fn example_address(&self) -> &str {
        "tz1VSUr8wwNhLAzempoch5d6hLRiTh8Cjcjb"
    }
}

impl Tezos {
    // SLIP-0010 Ed25519 derivation
    fn derive_ed25519_key(&self, seed: &[u8], path: &DerivationPath) -> ApiResult<[u8; 32]> {
        // Start with the master key
        let mut hasher = Sha512::new();
        hasher.update(b"ed25519 seed");
        hasher.update(seed);
        let mut key = hasher.finalize();
        
        // Derive through the path - Tezos uses all hardened derivation
        let indices = vec![
            0x80000000 + path.purpose,    // 44'
            0x80000000 + path.coin_type,  // 1729'
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