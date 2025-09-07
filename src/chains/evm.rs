use async_trait::async_trait;
use bitcoin::bip32::{Xpriv, DerivationPath as BtcDerivationPath, ChildNumber};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::Network;
use tiny_keccak::{Hasher, Keccak};

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::{ApiError, ApiResult};

/// EVM Chain implementation (works for Ethereum and all EVM-compatible L2s)
pub struct EvmChain {
    chain_type: ChainType,
}

impl EvmChain {
    pub fn new(chain_type: ChainType) -> Self {
        EvmChain { chain_type }
    }
}

#[async_trait]
impl Chain for EvmChain {
    fn info(&self) -> ChainInfo {
        get_chain_info(&self.chain_type)
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
            chain_type: self.chain_type.clone(),
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // All EVM chains use Ethereum's derivation path
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

impl EvmChain {
    // EIP-55 checksum address encoding
    fn to_checksum_address(&self, address_bytes: &[u8]) -> String {
        let address_hex = hex::encode(address_bytes);
        
        // Hash the lowercase address
        let mut hasher = Keccak::v256();
        hasher.update(address_hex.as_bytes());
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        let hash_hex = hex::encode(hash);
        
        // Apply checksum based on hash
        let mut checksummed = String::from("0x");
        for (i, c) in address_hex.chars().enumerate() {
            if c.is_alphabetic() {
                let hash_byte = u8::from_str_radix(&hash_hex[i..i+1], 16).unwrap();
                if hash_byte >= 8 {
                    checksummed.push(c.to_ascii_uppercase());
                } else {
                    checksummed.push(c);
                }
            } else {
                checksummed.push(c);
            }
        }
        
        checksummed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_evm_address_generation() {
        let seed = [0u8; 32];
        let chain = EvmChain::new(ChainType::Ethereum);
        let address = chain.generate_address(&seed, "", 0).await.unwrap();
        
        assert!(address.address.starts_with("0x"));
        assert_eq!(address.address.len(), 42);
        assert_eq!(address.chain_type, ChainType::Ethereum);
    }
    
    #[tokio::test]
    async fn test_base_address_generation() {
        let seed = [0u8; 32];
        let chain = EvmChain::new(ChainType::Base);
        let address = chain.generate_address(&seed, "", 0).await.unwrap();
        
        assert!(address.address.starts_with("0x"));
        assert_eq!(address.address.len(), 42);
        assert_eq!(address.chain_type, ChainType::Base);
    }
    
    #[tokio::test]
    async fn test_address_validation() {
        let chain = EvmChain::new(ChainType::Ethereum);
        
        // Valid addresses
        assert!(chain.validate_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb8").await);
        assert!(chain.validate_address("0x742d35cc6634c0532925a3b844bc9e7595f0beb8").await);
        
        // Invalid addresses
        assert!(!chain.validate_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb").await); // Too short
        assert!(!chain.validate_address("742d35Cc6634C0532925a3b844Bc9e7595f0bEb8").await); // No 0x
        assert!(!chain.validate_address("0xGGGG35Cc6634C0532925a3b844Bc9e7595f0bEb8").await); // Invalid hex
    }
}