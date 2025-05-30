use async_trait::async_trait;
use bitcoin::bip32::{Xpriv, DerivationPath as BtcDerivationPath, ChildNumber};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::Network;
use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use bech32::ToBase32;

use crate::core::{Chain, ChainInfo, ChainType, WalletAddress, DerivationPath, get_chain_info};
use crate::errors::{ApiError, ApiResult};

pub struct CosmosChain {
    chain_type: ChainType,
}

impl CosmosChain {
    pub fn new(chain_type: ChainType) -> Self {
        Self { chain_type }
    }
}

#[async_trait]
impl Chain for CosmosChain {
    fn info(&self) -> ChainInfo {
        get_chain_info(&self.chain_type)
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
        
        // Cosmos uses compressed public key
        let pubkey_bytes = secp_pubkey.serialize();
        
        // Hash public key with SHA256 then RIPEMD160
        let sha256_hash = Sha256::digest(&pubkey_bytes);
        let ripemd_hash = Ripemd160::digest(&sha256_hash);
        
        // Get the HRP from chain info
        let hrp = match &self.info().address_format {
            crate::core::AddressFormat::Bech32 { hrp } => hrp.clone(),
            _ => return Err(ApiError::CryptoError("Invalid address format".to_string())),
        };
        
        // Bech32 encode
        let address = bech32::encode(&hrp, ripemd_hash.to_base32(), bech32::Variant::Bech32)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        
        Ok(WalletAddress {
            address,
            chain_type: self.chain_type.clone(),
            chain_info: self.info(),
            derivation_path: path.to_string(),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    fn derivation_path(&self, index: u32) -> DerivationPath {
        let coin_type = self.info().coin_type;
        DerivationPath::new(44, coin_type, 0, 0, index)
    }

    async fn validate_address(&self, address: &str) -> bool {
        // Get expected HRP
        let info = self.info();
        let expected_hrp = match &info.address_format {
            crate::core::AddressFormat::Bech32 { hrp } => hrp,
            _ => return false,
        };
        
        // Validate bech32 encoding and HRP
        match bech32::decode(address) {
            Ok((hrp, _, _)) => hrp == *expected_hrp,
            Err(_) => false,
        }
    }

    fn example_address(&self) -> &str {
        match self.chain_type {
            ChainType::Cosmos => "cosmos1vqpjljwsynsn58dugz0w8ut7kun7t8ls2qkmsq",
            ChainType::Osmosis => "osmo1vqpjljwsynsn58dugz0w8ut7kun7t8ls8dswma",
            ChainType::Juno => "juno1vqpjljwsynsn58dugz0w8ut7kun7t8ls5qnd3f",
            ChainType::Secret => "secret1vqpjljwsynsn58dugz0w8ut7kun7t8lsj3n0rq",
            ChainType::Akash => "akash1vqpjljwsynsn58dugz0w8ut7kun7t8lsg8tcw6",
            ChainType::Sei => "sei1vqpjljwsynsn58dugz0w8ut7kun7t8ls4snhph",
            ChainType::Celestia => "celestia1vqpjljwsynsn58dugz0w8ut7kun7t8lsd8fhkg",
            ChainType::Injective => "inj1vqpjljwsynsn58dugz0w8ut7kun7t8lsj2w3wd",
            _ => "cosmos1vqpjljwsynsn58dugz0w8ut7kun7t8ls2qkmsq",
        }
    }
}