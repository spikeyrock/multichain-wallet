use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use sha2::{Sha512, Sha256, Digest};

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::ApiResult;

pub struct Algorand;

impl Algorand {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Algorand {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Algorand)
    }

    async fn generate_address(
        &self,
        seed: &[u8],
        _passphrase: &str,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Algorand uses Ed25519
        let path = self.derivation_path(index);
        
        // Derive key using SLIP-0010 for Ed25519
        let derived_key = self.derive_ed25519_key(seed, &path)?;
        
        let signing_key = SigningKey::from_bytes(&derived_key);
        let verifying_key = signing_key.verifying_key();
        
        // Algorand address is base32 encoded public key + checksum
        let public_key = verifying_key.as_bytes();
        
        // Calculate checksum: last 4 bytes of SHA512-256 hash
        let mut hasher = Sha512::new();
        hasher.update(public_key);
        let hash = hasher.finalize();
        let hash256 = Sha256::digest(&hash[..32]);
        let checksum = &hash256[28..32]; // Last 4 bytes
        
        // Combine public key and checksum
        let mut addr_bytes = Vec::new();
        addr_bytes.extend_from_slice(public_key);
        addr_bytes.extend_from_slice(checksum);
        
        // Base32 encode without padding
        let address = self.base32_encode_no_pad(&addr_bytes);
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Algorand,
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // Algorand uses m/44'/283'/0'/0/index
        DerivationPath::new(44, 283, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // Algorand addresses are 58 characters (base32 encoded)
        if address.len() != 58 {
            return false;
        }
        
        // Validate base32 characters (A-Z, 2-7)
        if !address.chars().all(|c| matches!(c, 'A'..='Z' | '2'..='7')) {
            return false;
        }
        
        // Decode and verify checksum
        match self.base32_decode(address) {
            Ok(bytes) => {
                if bytes.len() != 36 { // 32 bytes pubkey + 4 bytes checksum
                    return false;
                }
                
                let pubkey = &bytes[..32];
                let checksum = &bytes[32..];
                
                // Verify checksum
                let mut hasher = Sha512::new();
                hasher.update(pubkey);
                let hash = hasher.finalize();
                let hash256 = Sha256::digest(&hash[..32]);
                let expected_checksum = &hash256[28..32];
                
                checksum == expected_checksum
            }
            Err(_) => false,
        }
    }

    fn example_address(&self) -> &str {
        "7ZUECA7HFLZTXENRV24SHLU4AVPUTMTTDUFUBNBD64C73F3UHRTHAIOF6Q"
    }
}

impl Algorand {
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
            0x80000000 + path.coin_type,  // 283'
            0x80000000 + path.account,    // 0'
            path.change,                  // 0
            path.index,                   // index
        ];
        
        for (i, &index) in indices.iter().enumerate() {
            let mut hasher = Sha512::new();
            if i < 3 {
                hasher.update(&[0x00]); // hardened derivation for first 3 levels
            } else {
                hasher.update(&[0x02]); // non-hardened for last 2
            }
            hasher.update(&key[..32]); // private key part
            hasher.update(&index.to_be_bytes());
            key = hasher.finalize();
        }
        
        // Return the first 32 bytes as the private key
        let mut private_key = [0u8; 32];
        private_key.copy_from_slice(&key[..32]);
        Ok(private_key)
    }
    
    // Base32 encoding without padding
    fn base32_encode_no_pad(&self, data: &[u8]) -> String {
        const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
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
    
    // Base32 decoding
    fn base32_decode(&self, encoded: &str) -> Result<Vec<u8>, String> {
        const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
        let mut result = Vec::new();
        let mut bits = 0u32;
        let mut value = 0u32;
        
        for ch in encoded.chars() {
            let digit = ALPHABET.find(ch)
                .ok_or_else(|| format!("Invalid character: {}", ch))?;
            value = (value << 5) | digit as u32;
            bits += 5;
            
            if bits >= 8 {
                result.push((value >> (bits - 8)) as u8);
                bits -= 8;
            }
        }
        
        Ok(result)
    }
}