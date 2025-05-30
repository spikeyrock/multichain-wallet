use async_trait::async_trait;
use bitcoin::{Address, PublicKey};
use bitcoin::bip32::{Xpriv, DerivationPath as BtcDerivationPath, ChildNumber};
use bitcoin::secp256k1::Secp256k1;

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
        
        let bitcoin_pubkey = PublicKey {
            compressed: true,
            inner: secp_pubkey,
        };
        
        // Dogecoin uses P2PKH addresses like Bitcoin Legacy
        let address = Address::p2pkh(&bitcoin_pubkey, self.network);
        
        // For Dogecoin mainnet, we need to manually format the address
        // as the bitcoin library doesn't natively support Dogecoin
        let address_str = if self.network == Network::Bitcoin {
            // Convert Bitcoin address to Dogecoin format
            let address_bytes = address.to_string();
            // Replace the '1' prefix with 'D' for Dogecoin
            if address_bytes.starts_with('1') {
                format!("D{}", &address_bytes[1..])
            } else {
                address_bytes
            }
        } else {
            address.to_string()
        };
        
        Ok(WalletAddress {
            address: address_str,
            chain_type: ChainType::Dogecoin,
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        DerivationPath::new(44, 3, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // Dogecoin addresses start with 'D' for mainnet
        address.starts_with('D') && address.len() >= 26 && address.len() <= 35
    }

    fn example_address(&self) -> &str {
        "DH5yaieqoZN36fDVciNyRueRGvGLR3mr7L"
    }
}