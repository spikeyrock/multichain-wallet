use async_trait::async_trait;
use bitcoin::bip32::{Xpriv, DerivationPath as BtcDerivationPath, ChildNumber};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::Network;
use tiny_keccak::{Hasher, Keccak};

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::{ApiError, ApiResult};

pub struct Ethereum;

impl Ethereum {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Ethereum {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Ethereum)
    }

    async fn generate_address(
        &self,
        seed: &[u8],
        _passphrase: &str,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        let secp = Secp256k1::new();
        let master = Xpriv::new_master(Network::Bitcoin, seed)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;

        let path = self.derivation_path(index);
        let btc_path = BtcDerivationPath::from(vec![
            ChildNumber::from_hardened_idx(path.purpose).unwrap(),
            ChildNumber::from_hardened_idx(path.coin_type).unwrap(),
            ChildNumber::from_hardened_idx(path.account).unwrap(),
            ChildNumber::from_normal_idx(path.change).unwrap(),
            ChildNumber::from_normal_idx(path.index).unwrap(),
        ]);
        
        let child = master.derive_priv(&secp, &btc_path)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        
        let private_key = child.private_key;
        let secp_pubkey = private_key.public_key(&secp);
        
        // Get uncompressed public key bytes (65 bytes with 0x04 prefix)
        let pubkey_bytes = secp_pubkey.serialize_uncompressed();
        
        // Skip the 0x04 prefix for Ethereum address calculation
        let pubkey_no_prefix = &pubkey_bytes[1..];
        
        // Compute Ethereum address using Keccak256
        let mut hasher = Keccak::v256();
        hasher.update(pubkey_no_prefix);
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        
        // Take last 20 bytes of the hash
        let address_bytes = &hash[12..];
        
        // Apply EIP-55 checksum encoding
        let address = self.to_checksum_address(address_bytes);
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Ethereum,
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        DerivationPath::new(44, 60, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // Basic Ethereum address validation
        if !address.starts_with("0x") || address.len() != 42 {
            return false;
        }
        
        // Check if all characters after 0x are hex
        address[2..].chars().all(|c| c.is_ascii_hexdigit())
    }

    fn example_address(&self) -> &str {
        "0x742d35Cc6634C0532925a3b844Bc9e7595f9147a"
    }
}

impl Ethereum {
    // EIP-55 checksum address encoding
    fn to_checksum_address(&self, address_bytes: &[u8]) -> String {
        let address_hex = hex::encode(address_bytes);
        
        // Hash the lowercase address
        let mut hasher = Keccak::v256();
        hasher.update(address_hex.as_bytes());
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        let hash_hex = hex::encode(hash);
        
        // Apply checksum
        let mut checksum_address = String::with_capacity(42);
        checksum_address.push_str("0x");
        
        for (i, ch) in address_hex.chars().enumerate() {
            if ch.is_alphabetic() {
                let hash_byte = hash_hex.chars().nth(i).unwrap();
                if hash_byte as u8 >= b'8' {
                    checksum_address.push(ch.to_ascii_uppercase());
                } else {
                    checksum_address.push(ch.to_ascii_lowercase());
                }
            } else {
                checksum_address.push(ch);
            }
        }
        
        checksum_address
    }
}