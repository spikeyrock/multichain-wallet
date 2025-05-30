use async_trait::async_trait;
use bitcoin::bip32::{Xpriv, DerivationPath as BtcDerivationPath, ChildNumber};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::Network;
use sha2::{Sha256, Digest};
use ripemd::Ripemd160;

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::{ApiError, ApiResult};

pub struct Ripple;

impl Ripple {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Ripple {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Ripple)
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
        
        // XRP uses compressed public key
        let pubkey_bytes = secp_pubkey.serialize();
        
        // Hash public key with SHA256 then RIPEMD160
        let sha256_hash = Sha256::digest(&pubkey_bytes);
        let ripemd_hash = Ripemd160::digest(&sha256_hash);
        
        // Add XRP address prefix (0x00)
        let mut payload = vec![0x00];
        payload.extend_from_slice(&ripemd_hash);
        
        // Double SHA256 for checksum
        let checksum = Sha256::digest(&Sha256::digest(&payload));
        payload.extend_from_slice(&checksum[..4]);
        
        // Encode with XRP's base58 alphabet
        let address = bs58::encode(payload)
            .with_alphabet(bs58::Alphabet::RIPPLE)
            .into_string();
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Ripple,
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        DerivationPath::new(44, 144, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // XRP addresses start with 'r' and are 25-34 characters
        if !address.starts_with('r') || address.len() < 25 || address.len() > 34 {
            return false;
        }
        
        // Decode and verify checksum
        match bs58::decode(address).with_alphabet(bs58::Alphabet::RIPPLE).into_vec() {
            Ok(data) => {
                if data.len() < 25 {
                    return false;
                }
                
                let payload = &data[..data.len() - 4];
                let checksum = &data[data.len() - 4..];
                
                let computed_checksum = Sha256::digest(&Sha256::digest(payload));
                checksum == &computed_checksum[..4]
            }
            Err(_) => false,
        }
    }

    fn example_address(&self) -> &str {
        "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh"
    }
}