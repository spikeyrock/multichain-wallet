use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use sha2::{Sha512, Digest};
use hmac::{Hmac, Mac};
use blake2::{Blake2b, Digest as Blake2Digest};

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::{ApiError, ApiResult};

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
        // Sui uses Ed25519 with m/44'/784'/0'/0'/0' path
        let path = self.derivation_path(index);
        
        // Derive key using BIP32-Ed25519
        let derived_key = self.derive_ed25519_key(seed)?;
        
        let signing_key = SigningKey::from_bytes(&derived_key);
        let verifying_key = signing_key.verifying_key();
        
        // Sui address is derived from public key with Blake2b
        // Sui uses: flag(0x00) + pubkey
        let mut data_to_hash = Vec::new();
        data_to_hash.push(0x00); // Ed25519 signature scheme flag
        data_to_hash.extend_from_slice(verifying_key.as_bytes());
        
        let mut hasher = Blake2b::<typenum::U32>::new(); // 32 bytes = 256 bits
        hasher.update(&data_to_hash);
        let hash = hasher.finalize();
        
        // Format as 0x prefixed hex (lowercase)
        let address = format!("0x{}", hex::encode(hash));
        
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

    fn derivation_path(&self, _index: u32) -> DerivationPath {
        // Sui uses m/44'/784'/0'/0'/0' (all hardened, fixed path)
        DerivationPath::new(44, 784, 0, 0, 0)
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
    // BIP32-Ed25519 derivation for Sui (Trust Wallet style)
    fn derive_ed25519_key(&self, seed: &[u8]) -> ApiResult<[u8; 32]> {
        type HmacSha512 = Hmac<Sha512>;
        
        // Master key
        let mut mac = HmacSha512::new_from_slice(b"ed25519 seed")
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        mac.update(seed);
        let master = mac.finalize().into_bytes();
        
        let mut key = master[..32].to_vec();
        let mut chain_code = master[32..].to_vec();
        
        // Sui uses m/44'/784'/0'/0'/0' (all hardened)
        let indices = vec![
            0x80000000u32 + 44,    // 44'
            0x80000000u32 + 784,   // 784'
            0x80000000u32 + 0,     // 0'
            0x80000000u32 + 0,     // 0'
            0x80000000u32 + 0,     // 0'
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