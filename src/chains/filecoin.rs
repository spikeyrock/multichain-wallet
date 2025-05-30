use async_trait::async_trait;
use bitcoin::bip32::{Xpriv, DerivationPath as BtcDerivationPath, ChildNumber};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::Network;
use blake2::{Blake2b, Digest as Blake2Digest};

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::{ApiError, ApiResult};

pub struct Filecoin;

impl Filecoin {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Filecoin {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Filecoin)
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
        
        // Filecoin f1 address (secp256k1)
        // Protocol: 1 (secp256k1)
        let protocol = 1u8;
        
        // Get uncompressed public key
        let pubkey_bytes = secp_pubkey.serialize_uncompressed();
        
        // Blake2b-160 hash of the uncompressed public key
        let mut hasher = Blake2b::<typenum::U20>::new(); // 20 bytes = 160 bits
        hasher.update(&pubkey_bytes);
        let payload = hasher.finalize();
        
        // Create address bytes: protocol + payload
        let mut addr_bytes = vec![protocol];
        addr_bytes.extend_from_slice(&payload);
        
        // Calculate checksum using Blake2b
        let checksum = self.calculate_checksum(&addr_bytes)?;
        addr_bytes.extend_from_slice(&checksum);
        
        // Base32 encode with lowercase
        let encoded = self.base32_encode_lower(&addr_bytes[1..]); // Skip protocol byte for encoding
        let address = format!("f{}{}", protocol, encoded);
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Filecoin,
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // Filecoin uses m/44'/461'/0'/0/index
        DerivationPath::new(44, 461, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // Filecoin addresses start with 'f' followed by protocol number
        if !address.starts_with("f1") || address.len() < 10 {
            return false;
        }
        
        // Validate base32 characters (lowercase)
        let encoded_part = &address[2..];
        encoded_part.chars().all(|c| {
            matches!(c, 'a'..='z' | '2'..='7')
        })
    }

    fn example_address(&self) -> &str {
        "f1vuc4eu2wgsdnce2ngm4fprd5tijr7kjmttgcbzy"
    }
}

impl Filecoin {
    fn calculate_checksum(&self, data: &[u8]) -> ApiResult<[u8; 4]> {
        use blake2::{Blake2b, Digest};
        
        let mut hasher = Blake2b::<typenum::U4>::new();
        hasher.update(data);
        let result = hasher.finalize();
        
        let mut checksum = [0u8; 4];
        checksum.copy_from_slice(&result);
        Ok(checksum)
    }
    
    fn base32_encode_lower(&self, data: &[u8]) -> String {
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
}