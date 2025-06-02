// CORRECT SOLANA IMPLEMENTATION FOR TRUST WALLET
// Trust Wallet uses m/44'/501'/0' for first address (only 3 levels!)

use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use sha2::{Sha512, Digest};
use hmac::{Hmac, Mac};

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::{ApiError, ApiResult};

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
        // Trust Wallet uses m/44'/501'/index' for Solana (only 3 levels!)
        let path = self.derivation_path(index);
        let derived_key = self.derive_ed25519_key(seed, &path, index)?;
        
        let signing_key = SigningKey::from_bytes(&derived_key);
        let verifying_key = signing_key.verifying_key();
        
        let address = bs58::encode(verifying_key.as_bytes()).into_string();
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Solana,
            chain_info: self.info(),
            derivation_path: format!("m/44'/501'/{}'", index),
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // Trust Wallet uses index at position 3 for Solana
        DerivationPath::new(44, 501, index, 0, 0)
    }

    async fn validate_address(&self, address: &str) -> bool {
        match bs58::decode(address).into_vec() {
            Ok(bytes) => bytes.len() == 32,
            Err(_) => false,
        }
    }

    fn example_address(&self) -> &str {
        "ErD4UTnfDXEMkruTTFPKR3sbikboT3RhNMAKwcocH8gW"
    }
}

impl Solana {
    fn derive_ed25519_key(&self, seed: &[u8], _path: &DerivationPath, index: u32) -> ApiResult<[u8; 32]> {
        type HmacSha512 = Hmac<Sha512>;
        
        // Master key
        let mut mac = HmacSha512::new_from_slice(b"ed25519 seed")
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        mac.update(seed);
        let master = mac.finalize().into_bytes();
        
        let mut key = master[..32].to_vec();
        let mut chain_code = master[32..].to_vec();
        
        // Trust Wallet Solana: m/44'/501'/index' (only 3 levels, all hardened)
        let indices = vec![
            0x80000000u32 + 44,    // 44'
            0x80000000u32 + 501,   // 501'
            0x80000000u32 + index, // index'
        ];
        
        for idx in indices {
            let mut mac = HmacSha512::new_from_slice(&chain_code)
                .map_err(|e| ApiError::CryptoError(e.to_string()))?;
            
            // Hardened derivation
            mac.update(&[0x00]);
            mac.update(&key);
            mac.update(&idx.to_be_bytes());
            
            let result = mac.finalize().into_bytes();
            key = result[..32].to_vec();
            chain_code = result[32..].to_vec();
        }
        
        let mut private_key = [0u8; 32];
        private_key.copy_from_slice(&key);
        Ok(private_key)
    }
}