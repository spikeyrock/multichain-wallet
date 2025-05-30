use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use sha2::{Sha512, Digest};
use crc::{Crc, CRC_16_XMODEM};

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::ApiResult;

pub struct Stellar;

impl Stellar {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Stellar {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Stellar)
    }

    async fn generate_address(
        &self,
        seed: &[u8],
        _passphrase: &str,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Stellar uses Ed25519
        let path = self.derivation_path(index);
        
        // Derive key using SLIP-0010 for Ed25519
        let derived_key = self.derive_ed25519_key(seed, &path)?;
        
        let signing_key = SigningKey::from_bytes(&derived_key);
        let verifying_key = signing_key.verifying_key();
        
        // Stellar address encoding
        let mut payload = vec![6 << 3]; // Version byte for public key (G...)
        payload.extend_from_slice(verifying_key.as_bytes());
        
        // CRC16 checksum
        let crc = Crc::<u16>::new(&CRC_16_XMODEM);
        let checksum = crc.checksum(&payload);
        payload.extend_from_slice(&checksum.to_le_bytes());
        
        // Base32 encode
        let address = self.base32_encode(&payload);
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Stellar,
            chain_info: self.info(),
            derivation_path: path.to_string_all_hardened(),
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // Stellar uses m/44'/148'/index'
        DerivationPath::new(44, 148, index, 0, 0)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // Stellar addresses start with 'G' and are 56 characters
        if !address.starts_with('G') || address.len() != 56 {
            return false;
        }
        
        // Validate base32 characters
        address.chars().all(|c| {
            matches!(c, 'A'..='Z' | '2'..='7')
        })
    }

    fn example_address(&self) -> &str {
        "GBJCHUKZMTFSLOMNC7P4TS4VJJBTCYL3XKSOLXAUJSD56C4LHND5TWUQ"
    }
}

impl Stellar {
    // SLIP-0010 Ed25519 derivation
    fn derive_ed25519_key(&self, seed: &[u8], path: &DerivationPath) -> ApiResult<[u8; 32]> {
        // Start with the master key
        let mut hasher = Sha512::new();
        hasher.update(b"ed25519 seed");
        hasher.update(seed);
        let mut key = hasher.finalize();
        
        // Derive through the path - Stellar uses hardened derivation
        let indices = vec![
            0x80000000 + path.purpose,    // 44'
            0x80000000 + path.coin_type,  // 148'
            0x80000000 + path.account,    // index'
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
    
    // Stellar uses its own base32 encoding
    fn base32_encode(&self, data: &[u8]) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
        let mut result = String::new();
        
        let mut bits = 0u32;
        let mut value = 0u32;
        
        for &byte in data {
            value = (value << 8) | byte as u32;
            bits += 8;
            
            while bits >= 5 {
                let index = (value >> (bits - 5)) & 0x1F;
                result.push(CHARSET[index as usize] as char);
                bits -= 5;
            }
        }
        
        if bits > 0 {
            let index = (value << (5 - bits)) & 0x1F;
            result.push(CHARSET[index as usize] as char);
        }
        
        result
    }
}