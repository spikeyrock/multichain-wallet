use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use tiny_keccak::{Hasher, Keccak};
use num_bigint::BigUint;

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::{ApiError, ApiResult};

pub struct Monero;

impl Monero {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Monero {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Monero)
    }

    async fn generate_address(
        &self,
        seed: &[u8],
        _passphrase: &str,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Monero has a complex address generation scheme
        // This is a simplified version
        let path = self.derivation_path(index);
        
        // Monero uses its own key derivation
        let mut hasher = Keccak::v256();
        hasher.update(seed);
        hasher.update(&path.account.to_le_bytes());
        hasher.update(&path.index.to_le_bytes());
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        
        // Simplified: just use the hash as private key
        // Real Monero has spend and view keys
        let signing_key = SigningKey::from_bytes(&hash);
        let verifying_key = signing_key.verifying_key();
        
        // Simplified Monero address (network byte + spend pubkey + view pubkey + checksum)
        // Using network byte 18 for mainnet
        let mut addr_data = vec![18];
        addr_data.extend_from_slice(verifying_key.as_bytes());
        addr_data.extend_from_slice(verifying_key.as_bytes()); // Using same key for simplicity
        
        // Keccak256 checksum
        let mut hasher = Keccak::v256();
        hasher.update(&addr_data);
        let mut checksum = [0u8; 32];
        hasher.finalize(&mut checksum);
        addr_data.extend_from_slice(&checksum[..4]);
        
        // Base58 encode with Monero alphabet
        let address = self.base58_monero_encode(&addr_data)
            .map_err(|e| ApiError::CryptoError(e))?;
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Monero,
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // Monero uses m/44'/128'/account'/0/0
        DerivationPath::new(44, 128, index, 0, 0)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // Monero addresses start with '4' for mainnet and are 95 characters
        if !address.starts_with('4') || address.len() != 95 {
            return false;
        }
        
        // Try to decode
        self.base58_monero_decode(address).is_ok()
    }

    fn example_address(&self) -> &str {
        "44AFFq5kSiGBoZ4NMDwYtN18obc8AemS33DBLWs3H7otXft3XjrpDtQGv7SqSsaBYBb98uNbr2VBBEt7f2wfn3RVGQBEP3A"
    }
}

impl Monero {
    // Monero base58 encoding
    fn base58_monero_encode(&self, data: &[u8]) -> Result<String, String> {
        // Use the Monero base58 alphabet
        const ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
        
        // Convert to big number
        let mut num = BigUint::from_bytes_be(data);
        let base = BigUint::from(58u32);
        
        let mut result = Vec::new();
        
        while num > BigUint::from(0u32) {
            let remainder = &num % &base;
            let digits = remainder.to_u32_digits();
            if !digits.is_empty() {
                result.push(ALPHABET[digits[0] as usize]);
            }
            num /= &base;
        }
        
        // Handle leading zeros
        for &byte in data {
            if byte == 0 {
                result.push(ALPHABET[0]);
            } else {
                break;
            }
        }
        
        result.reverse();
        Ok(String::from_utf8(result).unwrap())
    }
    
    // Monero base58 decoding
    fn base58_monero_decode(&self, encoded: &str) -> Result<Vec<u8>, String> {
        const ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
        
        let base = BigUint::from(58u32);
        let mut num = BigUint::from(0u32);
        
        for ch in encoded.chars() {
            let digit = ALPHABET.find(ch)
                .ok_or_else(|| format!("Invalid character: {}", ch))?;
            num = num * &base + BigUint::from(digit);
        }
        
        let bytes = num.to_bytes_be();
        
        // Count leading 1s (zeros in the result)
        let leading_zeros = encoded.chars().take_while(|&c| c == '1').count();
        let mut result = vec![0u8; leading_zeros];
        result.extend_from_slice(&bytes);
        
        Ok(result)
    }
}