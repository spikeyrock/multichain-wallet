use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use blake2::{Blake2b512, Digest as Blake2Digest};

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::ApiResult;

pub struct Polkadot;

impl Polkadot {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Chain for Polkadot {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::Polkadot)
    }

    async fn generate_address(
        &self,
        seed: &[u8],
        _passphrase: &str,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Polkadot uses Ed25519 or Sr25519, we'll use Ed25519 for simplicity
        let path = self.derivation_path(index);
        
        // Derive key
        let derived_key = self.derive_polkadot_key(seed, &path)?;
        
        let signing_key = SigningKey::from_bytes(&derived_key);
        let verifying_key = signing_key.verifying_key();
        
        // Polkadot SS58 address encoding
        let address = self.to_ss58_address(verifying_key.as_bytes(), 0)?; // 0 = Polkadot network
        
        Ok(WalletAddress {
            address,
            chain_type: ChainType::Polkadot,
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        // Polkadot uses m/44'/354'/0'/0/index
        DerivationPath::new(44, 354, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // Polkadot addresses start with '1' and use SS58 encoding
        if !address.starts_with('1') || address.len() < 46 || address.len() > 48 {
            return false;
        }
        
        // Validate SS58 encoding
        self.decode_ss58_address(address).is_ok()
    }

    fn example_address(&self) -> &str {
        "1FRMM8PEiWXYax7rpS6X4XZX1aAAxSWx1CrKTyrVYhV24fg"
    }
}

impl Polkadot {
    // Simplified Polkadot key derivation
    fn derive_polkadot_key(&self, seed: &[u8], path: &DerivationPath) -> ApiResult<[u8; 32]> {
        // Use Blake2b for key derivation (Polkadot style)
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
    
    // SS58 address encoding
    fn to_ss58_address(&self, public_key: &[u8], network_id: u8) -> ApiResult<String> {
        let mut data = vec![network_id];
        data.extend_from_slice(public_key);
        
        // SS58 checksum
        let mut hasher = Blake2b512::new();
        hasher.update(b"SS58PRE");
        hasher.update(&data);
        let hash = hasher.finalize();
        data.extend_from_slice(&hash[..2]);
        
        Ok(bs58::encode(data).into_string())
    }
    
    // SS58 address decoding
    fn decode_ss58_address(&self, address: &str) -> Result<Vec<u8>, String> {
        let data = bs58::decode(address)
            .into_vec()
            .map_err(|e| e.to_string())?;
            
        if data.len() < 35 {
            return Err("Invalid address length".to_string());
        }
        
        let payload = &data[..data.len() - 2];
        let checksum = &data[data.len() - 2..];
        
        // Verify checksum
        let mut hasher = Blake2b512::new();
        hasher.update(b"SS58PRE");
        hasher.update(payload);
        let hash = hasher.finalize();
        
        if checksum != &hash[..2] {
            return Err("Invalid checksum".to_string());
        }
        
        Ok(payload[1..33].to_vec())
    }
}