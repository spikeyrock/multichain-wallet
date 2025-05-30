use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use blake2::{Blake2b512, Blake2s256, Digest};
use bech32::ToBase32;

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::{ApiError, ApiResult};

pub struct Cardano;

impl Cardano {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Cardano {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Cardano)
    }

    async fn generate_address(
        &self,
        seed: &[u8],
        _passphrase: &str,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Cardano uses Ed25519 with a specific derivation scheme
        let path = self.derivation_path(index);
        
        // Derive key - simplified version
        let derived_key = self.derive_cardano_key(seed, &path)?;
        
        let signing_key = SigningKey::from_bytes(&derived_key);
        let verifying_key = signing_key.verifying_key();
        
        // Simplified Cardano address (Shelley era payment address)
        // In production, you'd need proper address construction with network tag, etc.
        let mut addr_bytes = vec![0x01]; // Mainnet payment key
        
        // Hash the public key
        let mut hasher = Blake2s256::new();
        hasher.update(verifying_key.as_bytes());
        let pub_key_hash = hasher.finalize();
        addr_bytes.extend_from_slice(&pub_key_hash[..28]);
        
        // Bech32 encode with "addr" prefix
        let address = bech32::encode("addr", addr_bytes.to_base32(), bech32::Variant::Bech32)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Cardano,
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // Cardano uses m/1852'/1815'/0'/0/{index}
        DerivationPath::new(1852, 1815, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // Basic Cardano address validation
        if !address.starts_with("addr1") {
            return false;
        }
        
        // Try to decode as bech32
        bech32::decode(address).is_ok()
    }

    fn example_address(&self) -> &str {
        "addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x"
    }
}

impl Cardano {
    // Simplified Cardano key derivation
    fn derive_cardano_key(&self, seed: &[u8], path: &DerivationPath) -> ApiResult<[u8; 32]> {
        // Use Blake2b-512 for key derivation
        let mut hasher = Blake2b512::new();
        hasher.update(seed);
        hasher.update(&path.purpose.to_le_bytes());
        hasher.update(&path.coin_type.to_le_bytes());
        hasher.update(&path.account.to_le_bytes());
        hasher.update(&path.change.to_le_bytes());
        hasher.update(&path.index.to_le_bytes());
        let hash = hasher.finalize();
        
        let mut private_key = [0u8; 32];
        private_key.copy_from_slice(&hash[..32]);
        Ok(private_key)
    }
}