use async_trait::async_trait;
use bitcoin::bip32::{Xpriv, DerivationPath as BtcDerivationPath, ChildNumber};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::Network;
use tiny_keccak::{Hasher, Keccak};
use sha2::{Sha256, Digest};

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::{ApiError, ApiResult};

pub struct Tron;

impl Tron {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Tron {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Tron)
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
        
        // Skip the 0x04 prefix for address calculation
        let pubkey_no_prefix = &pubkey_bytes[1..];
        
        // Compute address using Keccak256
        let mut hasher = Keccak::v256();
        hasher.update(pubkey_no_prefix);
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        
        // Take last 20 bytes and add Tron prefix (0x41)
        let mut address_bytes = vec![0x41];
        address_bytes.extend_from_slice(&hash[12..]);
        
        // Double SHA256 for checksum
        let checksum = Sha256::digest(&Sha256::digest(&address_bytes));
        address_bytes.extend_from_slice(&checksum[..4]);
        
        // Base58 encode
        let address = bs58::encode(address_bytes).into_string();
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Tron,
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        DerivationPath::new(44, 195, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // TRON addresses start with 'T' and are 34 characters
        if !address.starts_with('T') || address.len() != 34 {
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
        "TLBaRhANQoJFTqre9Nf1mjDRHMHEz4LvKE"
    }
}