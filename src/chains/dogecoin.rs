// src/chains/dogecoin.rs
use async_trait::async_trait;
use bitcoin::PublicKey;
use bitcoin::bip32::{Xpriv, DerivationPath as BtcDerivationPath, ChildNumber};
use bitcoin::secp256k1::Secp256k1;
use sha2::{Sha256, Digest};
use ripemd::Ripemd160;

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::{ApiError, ApiResult};

// Re-export Network so it can be used in mod.rs
pub use bitcoin::Network;

// Dogecoin implementation
pub struct Dogecoin {
    network: Network,
}

impl Dogecoin {
    pub fn new(network: Network) -> Self {
        Self { network }
    }
}

#[async_trait]
impl Chain for Dogecoin {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Dogecoin)
    }

    async fn generate_address(
        &self,
        seed: &[u8],
        _passphrase: &str,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        let secp = Secp256k1::new();
        let master = Xpriv::new_master(self.network, seed)
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
        
        // Get compressed public key
        let pubkey_bytes = secp_pubkey.serialize();
        
        // Dogecoin address generation (P2PKH)
        // 1. SHA256 hash of public key
        let sha256_hash = Sha256::digest(&pubkey_bytes);
        
        // 2. RIPEMD160 hash of SHA256 result
        let ripemd_hash = Ripemd160::digest(&sha256_hash);
        
        // 3. Add Dogecoin version byte (0x1E for mainnet)
        let mut payload = vec![0x1E];
        payload.extend_from_slice(&ripemd_hash);
        
        // 4. Double SHA256 for checksum
        let checksum = Sha256::digest(&Sha256::digest(&payload));
        payload.extend_from_slice(&checksum[..4]);
        
        // 5. Base58 encode
        let address = bs58::encode(payload).into_string();
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Dogecoin,
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // Dogecoin uses m/44'/3'/0'/0/{index}
        DerivationPath::new(44, 3, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // Dogecoin addresses start with 'D' for mainnet and are 34 characters
        if !address.starts_with('D') || address.len() != 34 {
            return false;
        }
        
        // Decode and verify checksum
        match bs58::decode(address).into_vec() {
            Ok(data) => {
                if data.len() != 25 {
                    return false;
                }
                
                let payload = &data[..21];
                let checksum = &data[21..];
                
                let computed_checksum = Sha256::digest(&Sha256::digest(payload));
                checksum == &computed_checksum[..4]
            }
            Err(_) => false,
        }
    }

    fn example_address(&self) -> &str {
        "DH5yaieqoZN36fDVciNyRueRGvGLR3mr7L"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bip39::Mnemonic;
    
    #[tokio::test]
    async fn test_dogecoin_address_generation() {
        // Test with a known mnemonic
        let mnemonic = "test walk nut penalty hip pave soap entry language right filter choice";
        let mnemonic = Mnemonic::parse(mnemonic).unwrap();
        let seed = mnemonic.to_seed("");
        
        let dogecoin = Dogecoin::new(Network::Bitcoin); // Mainnet
        let wallet = dogecoin.generate_address(&seed, "", 0).await.unwrap();
        
        println!("Generated Dogecoin address: {}", wallet.address);
        assert!(wallet.address.starts_with('D'));
        assert_eq!(wallet.address.len(), 34);
        
        // Verify the derivation path
        assert_eq!(wallet.derivation_path, "m/44'/3'/0'/0/0");
    }
    
    #[tokio::test]
    async fn test_dogecoin_address_validation() {
        let dogecoin = Dogecoin::new(Network::Bitcoin);
        
        // Valid Dogecoin addresses
        assert!(dogecoin.validate_address("DH5yaieqoZN36fDVciNyRueRGvGLR3mr7L").await);
        assert!(dogecoin.validate_address("DBKh7QAP9gkXncVK32jtfae4QXChPwsyKH").await);
        
        // Invalid addresses
        assert!(!dogecoin.validate_address("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa").await); // Bitcoin address
        assert!(!dogecoin.validate_address("DInvalidAddress").await);
        assert!(!dogecoin.validate_address("").await);
    }
}