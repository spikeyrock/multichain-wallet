use async_trait::async_trait;
use bitcoin::bip32::{Xpriv, DerivationPath as BtcDerivationPath, ChildNumber};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::Network;

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::{ApiError, ApiResult};

pub struct Eos;

impl Eos {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Eos {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Eos)
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
        
        // EOS uses a different format:
        // 1. Legacy format: "EOS" + base58(compressed_pubkey + checksum)
        let compressed_pubkey = secp_pubkey.serialize();
        
        // Create the full public key string with "EOS" prefix
        let address = format!("EOS{}", self.encode_eos_pubkey(&compressed_pubkey)?);
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Eos,
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(compressed_pubkey),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // EOS uses m/44'/194'/0'/0/index
        DerivationPath::new(44, 194, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // EOS has account names (12 chars) but we're generating public keys
        // Legacy format: EOS + 50 chars
        if address.starts_with("EOS") && address.len() == 53 {
            // Validate base58 characters
            let key_part = &address[3..];
            key_part.chars().all(|c| {
                matches!(c, '1'..='9' | 'A'..='H' | 'J'..='N' | 'P'..='Z' | 'a'..='k' | 'm'..='z')
            })
        } else {
            false
        }
    }

    fn example_address(&self) -> &str {
        "EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV"
    }
}

impl Eos {
    fn encode_eos_pubkey(&self, pubkey: &[u8]) -> ApiResult<String> {
        use ripemd::Ripemd160;
        use sha2::{Sha256, Digest};
        
        // Add suffix for checksum calculation
        let mut check_data = pubkey.to_vec();
        check_data.extend_from_slice(b"K1");
        
        // Calculate checksum: RIPEMD160(SHA256(data + suffix))
        let sha_hash = Sha256::digest(&check_data);
        let checksum = Ripemd160::digest(&sha_hash);
        
        // Append first 4 bytes of checksum to pubkey
        let mut final_data = pubkey.to_vec();
        final_data.extend_from_slice(&checksum[..4]);
        
        // Base58 encode
        Ok(bs58::encode(final_data).into_string())
    }
}