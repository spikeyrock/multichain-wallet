use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use sha2::{Sha512, Digest};
use crc::{Crc, CRC_16_XMODEM};
use base64::{Engine as _, engine::general_purpose};

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::ApiResult;

pub struct Ton;

impl Ton {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Ton {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Ton)
    }

    async fn generate_address(
        &self,
        seed: &[u8],
        _passphrase: &str,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // TON uses Ed25519
        let path = self.derivation_path(index);
        
        // Derive key using SLIP-0010 for Ed25519
        let derived_key = self.derive_ed25519_key(seed, &path)?;
        
        let signing_key = SigningKey::from_bytes(&derived_key);
        let verifying_key = signing_key.verifying_key();
        
        // TON address construction (simplified)
        // Real TON addresses are more complex with workchain and account IDs
        let mut address_data = vec![0x00]; // Workchain 0
        address_data.extend_from_slice(verifying_key.as_bytes());
        
        // Add CRC16 checksum
        let crc = Crc::<u16>::new(&CRC_16_XMODEM);
        let checksum = crc.checksum(&address_data);
        address_data.extend_from_slice(&checksum.to_be_bytes());
        
        // Base64 URL-safe encode with "EQ" prefix
        let encoded = general_purpose::URL_SAFE_NO_PAD.encode(&address_data);
        let address = format!("EQ{}", encoded);
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Ton,
            chain_info: self.info(),
            derivation_path: path.to_string_all_hardened(),
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // TON uses m/44'/607'/0'/0'/index'
        DerivationPath::new(44, 607, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // TON addresses start with 'EQ' and are base64 encoded
        if !address.starts_with("EQ") || address.len() < 48 {
            return false;
        }
        
        // Try to decode base64
        general_purpose::URL_SAFE_NO_PAD.decode(&address[2..]).is_ok()
    }

    fn example_address(&self) -> &str {
        "EQBvW8Z5huBkMJYdnfAEM5JqTNkuWX3diqYENkWsIL0XggGG"
    }
}

impl Ton {
    // SLIP-0010 Ed25519 derivation
    fn derive_ed25519_key(&self, seed: &[u8], path: &DerivationPath) -> ApiResult<[u8; 32]> {
        // Start with the master key
        let mut hasher = Sha512::new();
        hasher.update(b"ed25519 seed");
        hasher.update(seed);
        let mut key = hasher.finalize();
        
        // Derive through the path - TON uses all hardened derivation
        let indices = vec![
            0x80000000 + path.purpose,    // 44'
            0x80000000 + path.coin_type,  // 607'
            0x80000000 + path.account,    // 0'
            0x80000000 + path.change,     // 0'
            0x80000000 + path.index,      // index'
        ];
        
        for index in indices {
            let mut hasher = Sha512::new();
            hasher.update(&[0x00]); // hardened derivation
            hasher.update(&key[..32]); // private key part
            hasher.update(&index.to_be_bytes());
            key = hasher.finalize();
        }
        
        // Return the first 32 bytes as the private key
        let mut private_key = [0u8; 32];
        private_key.copy_from_slice(&key[..32]);
        Ok(private_key)
    }
}