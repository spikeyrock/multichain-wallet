use async_trait::async_trait;
use sha2::{Sha256, Sha224, Digest};
use crc::{Crc, CRC_32_ISO_HDLC};

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::ApiResult;

pub struct InternetComputer;

impl InternetComputer {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for InternetComputer {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::InternetComputer)
    }

    async fn generate_address(
        &self,
        seed: &[u8],
        _passphrase: &str,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // ICP uses Ed25519 but with principal IDs
        let path = self.derivation_path(index);
        
        // Derive key (simplified - real ICP uses proper Ed25519)
        let mut hasher = Sha256::new();
        hasher.update(seed);
        hasher.update(&path.purpose.to_le_bytes());
        hasher.update(&path.coin_type.to_le_bytes());
        hasher.update(&path.account.to_le_bytes());
        hasher.update(&path.change.to_le_bytes());
        hasher.update(&path.index.to_le_bytes());
        let private_key = hasher.finalize();
        
        // Generate public key
        let mut pub_hasher = Sha256::new();
        pub_hasher.update(&private_key);
        pub_hasher.update(b"icp-public");
        let public_key = pub_hasher.finalize();
        
        // Generate principal ID
        let principal_id = self.derive_principal_id(&public_key)?;
        
        Ok(WalletAddress {
            address: principal_id,
            chain_type: ChainType::InternetComputer,
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(&public_key),
            private_key: hex::encode(&private_key),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // ICP uses m/44'/223'/0'/0/index
        DerivationPath::new(44, 223, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // Principal IDs are base32 encoded with dashes
        // Format: xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxx
        let parts: Vec<&str> = address.split('-').collect();
        
        // Should have multiple parts
        if parts.is_empty() || parts.len() > 11 {
            return false;
        }
        
        // Each part should be 5 characters except possibly the last
        for (i, part) in parts.iter().enumerate() {
            if i < parts.len() - 1 {
                if part.len() != 5 {
                    return false;
                }
            } else {
                // Last part can be 1-5 characters
                if part.is_empty() || part.len() > 5 {
                    return false;
                }
            }
            
            // Validate base32 characters (lowercase)
            if !part.chars().all(|c| matches!(c, 'a'..='z' | '2'..='7')) {
                return false;
            }
        }
        
        true
    }

    fn example_address(&self) -> &str {
        "rdmx6-jaaaa-aaaaa-aaadq-cai"
    }
}

impl InternetComputer {
    fn derive_principal_id(&self, public_key: &[u8]) -> ApiResult<String> {
        // Principal ID derivation
        // 1. Hash the DER-encoded public key
        let mut hasher = Sha224::new();
        hasher.update(b"\x0A"); // DER prefix for anonymous principal
        hasher.update(public_key);
        let hash = hasher.finalize();
        
        // 2. Add CRC32 checksum
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let checksum = crc.checksum(&hash);
        
        let mut principal_bytes = Vec::new();
        principal_bytes.extend_from_slice(&checksum.to_be_bytes());
        principal_bytes.extend_from_slice(&hash);
        
        // 3. Base32 encode (custom alphabet)
        let encoded = self.base32_encode_principal(&principal_bytes);
        
        // 4. Format with dashes
        self.format_principal_id(&encoded)
    }
    
    fn base32_encode_principal(&self, data: &[u8]) -> String {
        const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz234567";
        let mut result = String::new();
        let mut bits = 0u32;
        let mut value = 0u32;
        
        for &byte in data {
            value = (value << 8) | byte as u32;
            bits += 8;
            
            while bits >= 5 {
                let index = (value >> (bits - 5)) & 0x1F;
                result.push(ALPHABET[index as usize] as char);
                bits -= 5;
            }
        }
        
        if bits > 0 {
            let index = (value << (5 - bits)) & 0x1F;
            result.push(ALPHABET[index as usize] as char);
        }
        
        result
    }
    
    fn format_principal_id(&self, encoded: &str) -> ApiResult<String> {
        // Insert dashes every 5 characters
        let mut formatted = String::new();
        let chars: Vec<char> = encoded.chars().collect();
        
        for (i, chunk) in chars.chunks(5).enumerate() {
            if i > 0 {
                formatted.push('-');
            }
            formatted.extend(chunk);
        }
        
        Ok(formatted)
    }
}