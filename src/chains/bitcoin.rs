use async_trait::async_trait;
use bitcoin::{Address, PublicKey, XOnlyPublicKey};
use bitcoin::bip32::{Xpriv, DerivationPath as BtcDerivationPath, ChildNumber};
use bitcoin::secp256k1::Secp256k1;

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::{ApiError, ApiResult};

// Re-export Network so it can be used in mod.rs
pub use bitcoin::Network;

// Bitcoin Legacy implementation
pub struct BitcoinLegacy {
    network: Network,
}

impl BitcoinLegacy {
    pub fn new(network: Network) -> Self {
        Self { network }
    }
}

#[async_trait]
impl Chain for BitcoinLegacy {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::BitcoinLegacy)
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
        
        let address = Address::p2pkh(&bitcoin_pubkey, self.network);
        
        Ok(WalletAddress {
            address: address.to_string(),
            chain_type: ChainType::BitcoinLegacy,
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        DerivationPath::new(44, 0, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        use std::str::FromStr;
        match Address::from_str(address) {
            Ok(addr) => {
                // Check if it's valid for the network and starts with 1
                match addr.require_network(self.network) {
                    Ok(_) => address.starts_with('1'),
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }

    fn example_address(&self) -> &str {
        "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
    }
}

// Bitcoin SegWit implementation
pub struct BitcoinSegwit {
    network: Network,
}

impl BitcoinSegwit {
    pub fn new(network: Network) -> Self {
        Self { network }
    }
}

#[async_trait]
impl Chain for BitcoinSegwit {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::BitcoinSegwit)
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
        
        let address = Address::p2wpkh(&bitcoin_pubkey, self.network)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        
        Ok(WalletAddress {
            address: address.to_string(),
            chain_type: ChainType::BitcoinSegwit,
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        DerivationPath::new(84, 0, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        use std::str::FromStr;
        match Address::from_str(address) {
            Ok(addr) => {
                // Check if it's valid for the network and starts with bc1q
                match addr.require_network(self.network) {
                    Ok(_) => address.starts_with("bc1q"),
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }

    fn example_address(&self) -> &str {
        "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"
    }
}

// Bitcoin Taproot implementation
pub struct BitcoinTaproot {
    network: Network,
}

impl BitcoinTaproot {
    pub fn new(network: Network) -> Self {
        Self { network }
    }
}

#[async_trait]
impl Chain for BitcoinTaproot {
    fn info(&self) -> ChainInfo {
        get_chain_info(&ChainType::BitcoinTaproot)
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
        let x_only_pubkey = XOnlyPublicKey::from(secp_pubkey);
        
        let address = Address::p2tr(&secp, x_only_pubkey, None, self.network);
        
        Ok(WalletAddress {
            address: address.to_string(),
            chain_type: ChainType::BitcoinTaproot,
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        DerivationPath::new(86, 0, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        use std::str::FromStr;
        match Address::from_str(address) {
            Ok(addr) => {
                // Check if it's valid for the network and starts with bc1p
                match addr.require_network(self.network) {
                    Ok(_) => address.starts_with("bc1p"),
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }

    fn example_address(&self) -> &str {
        "bc1p5d7rjq7g6rdk2yhzks9smlaqtedr4dekq08ge8ztwac72sfr9rusxg3297"
    }
}