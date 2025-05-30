use bip39::{Language, Mnemonic};
use bitcoin::{Network, Address, PublicKey, XOnlyPublicKey};
use bitcoin::bip32::{Xpriv, DerivationPath, ChildNumber};
use bitcoin::secp256k1::Secp256k1;
use rand::RngCore;

use crate::api::models::{AddressType, WalletAddress};
use crate::errors::{ApiError, ApiResult};

pub struct WalletService {}

impl WalletService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn generate_mnemonic(
        &self,
        language_str: &str,
        word_count: u32,
    ) -> ApiResult<String> {
        let language = self.parse_language(language_str)?;
        
        // Generate entropy based on word count
        let entropy_bytes = match word_count {
            12 => 16, // 128 bits
            15 => 20, // 160 bits
            18 => 24, // 192 bits
            21 => 28, // 224 bits
            24 => 32, // 256 bits
            _ => return Err(ApiError::InvalidWordCount(word_count)),
        };

        // Generate random entropy
        let mut entropy = vec![0u8; entropy_bytes];
        rand::thread_rng().fill_bytes(&mut entropy);

        // Create mnemonic from entropy
        let mnemonic = Mnemonic::from_entropy_in(language, &entropy)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;

        Ok(mnemonic.to_string())
    }

    pub async fn validate_mnemonic(&self, mnemonic_str: &str, language_str: &str) -> (bool, u32) {
        let language = match self.parse_language(language_str) {
            Ok(lang) => lang,
            Err(_) => return (false, 0),
        };

        match Mnemonic::parse_in_normalized(language, mnemonic_str) {
            Ok(mnemonic) => {
                let word_count = mnemonic.word_count() as u32;
                (true, word_count)
            }
            Err(_) => (false, 0),
        }
    }

    pub async fn generate_wallet_address(
        &self,
        mnemonic_str: &str,
        passphrase: &str,
        address_type: &AddressType,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Parse mnemonic
        let mnemonic = Mnemonic::parse(mnemonic_str)
            .map_err(|_| ApiError::InvalidMnemonic)?;

        // Generate seed from mnemonic and passphrase
        let seed = mnemonic.to_seed(passphrase);

        // Create secp256k1 context
        let secp = Secp256k1::new();

        // Create master key from seed
        let master = Xpriv::new_master(Network::Bitcoin, &seed)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;

        // Generate address based on type
        match address_type {
            AddressType::BitcoinTaproot => self.generate_bitcoin_taproot(&secp, master, index).await,
            AddressType::BitcoinSegwit => self.generate_bitcoin_segwit(&secp, master, index).await,
            AddressType::BitcoinLegacy => self.generate_bitcoin_legacy(&secp, master, index).await,
            AddressType::Ethereum => self.generate_ethereum(&secp, master, index).await,
        }
    }

    async fn generate_bitcoin_taproot(
        &self,
        secp: &Secp256k1<bitcoin::secp256k1::All>,
        master: Xpriv,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // m/86'/0'/0'/0/{index}
        let path = DerivationPath::from(vec![
            ChildNumber::from_hardened_idx(86).unwrap(),
            ChildNumber::from_hardened_idx(0).unwrap(),
            ChildNumber::from_hardened_idx(0).unwrap(),
            ChildNumber::from_normal_idx(0).unwrap(),
            ChildNumber::from_normal_idx(index).unwrap(),
        ]);
        
        let child = master.derive_priv(secp, &path)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        
        let private_key = child.private_key;
        let secp_pubkey = private_key.public_key(secp);
        let x_only_pubkey = XOnlyPublicKey::from(secp_pubkey);
        
        let address = Address::p2tr(&secp, x_only_pubkey, None, Network::Bitcoin);
        
        Ok(WalletAddress {
            address: address.to_string(),
            address_type: AddressType::BitcoinTaproot,
            derivation_path: format!("m/86'/0'/0'/0/{}", index),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    async fn generate_bitcoin_segwit(
        &self,
        secp: &Secp256k1<bitcoin::secp256k1::All>,
        master: Xpriv,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // m/84'/0'/0'/0/{index}
        let path = DerivationPath::from(vec![
            ChildNumber::from_hardened_idx(84).unwrap(),
            ChildNumber::from_hardened_idx(0).unwrap(),
            ChildNumber::from_hardened_idx(0).unwrap(),
            ChildNumber::from_normal_idx(0).unwrap(),
            ChildNumber::from_normal_idx(index).unwrap(),
        ]);
        
        let child = master.derive_priv(secp, &path)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        
        let private_key = child.private_key;
        let secp_pubkey = private_key.public_key(secp);
        
        // Convert secp256k1 public key to bitcoin public key
        let bitcoin_pubkey = PublicKey {
            compressed: true,
            inner: secp_pubkey,
        };
        
        let address = Address::p2wpkh(&bitcoin_pubkey, Network::Bitcoin)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        
        Ok(WalletAddress {
            address: address.to_string(),
            address_type: AddressType::BitcoinSegwit,
            derivation_path: format!("m/84'/0'/0'/0/{}", index),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    async fn generate_bitcoin_legacy(
        &self,
        secp: &Secp256k1<bitcoin::secp256k1::All>,
        master: Xpriv,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // m/44'/0'/0'/0/{index}
        let path = DerivationPath::from(vec![
            ChildNumber::from_hardened_idx(44).unwrap(),
            ChildNumber::from_hardened_idx(0).unwrap(),
            ChildNumber::from_hardened_idx(0).unwrap(),
            ChildNumber::from_normal_idx(0).unwrap(),
            ChildNumber::from_normal_idx(index).unwrap(),
        ]);
        
        let child = master.derive_priv(secp, &path)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        
        let private_key = child.private_key;
        let secp_pubkey = private_key.public_key(secp);
        
        // Convert secp256k1 public key to bitcoin public key
        let bitcoin_pubkey = PublicKey {
            compressed: true,
            inner: secp_pubkey,
        };
        
        let address = Address::p2pkh(&bitcoin_pubkey, Network::Bitcoin);
        
        Ok(WalletAddress {
            address: address.to_string(),
            address_type: AddressType::BitcoinLegacy,
            derivation_path: format!("m/44'/0'/0'/0/{}", index),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    async fn generate_ethereum(
        &self,
        secp: &Secp256k1<bitcoin::secp256k1::All>,
        master: Xpriv,
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // m/44'/60'/0'/0/{index}
        let path = DerivationPath::from(vec![
            ChildNumber::from_hardened_idx(44).unwrap(),
            ChildNumber::from_hardened_idx(60).unwrap(),
            ChildNumber::from_hardened_idx(0).unwrap(),
            ChildNumber::from_normal_idx(0).unwrap(),
            ChildNumber::from_normal_idx(index).unwrap(),
        ]);
        
        let child = master.derive_priv(secp, &path)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        
        let private_key = child.private_key;
        let secp_pubkey = private_key.public_key(secp);
        
        // Get uncompressed public key bytes (65 bytes with 0x04 prefix)
        let pubkey_bytes = secp_pubkey.serialize_uncompressed();
        
        // Skip the 0x04 prefix for Ethereum address calculation
        let pubkey_no_prefix = &pubkey_bytes[1..];
        
        // Compute Ethereum address using Keccak256
        use tiny_keccak::{Hasher, Keccak};
        let mut hasher = Keccak::v256();
        hasher.update(pubkey_no_prefix);
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        
        // Take last 20 bytes of the hash
        let address_bytes = &hash[12..];
        let address = format!("0x{}", hex::encode(address_bytes));
        
        Ok(WalletAddress {
            address,
            address_type: AddressType::Ethereum,
            derivation_path: format!("m/44'/60'/0'/0/{}", index),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    pub async fn batch_generate_wallet_addresses(
        &self,
        mnemonic_str: &str,
        passphrase: &str,
        address_types: &[AddressType],
        start_index: u32,
        count: u32,
    ) -> ApiResult<Vec<WalletAddress>> {
        let mut addresses = Vec::new();
        
        for address_type in address_types {
            for i in start_index..(start_index + count) {
                let wallet = self.generate_wallet_address(
                    mnemonic_str,
                    passphrase,
                    address_type,
                    i,
                ).await?;
                addresses.push(wallet);
            }
        }
        
        Ok(addresses)
    }

    fn parse_language(&self, language: &str) -> ApiResult<Language> {
        match language.to_lowercase().as_str() {
            "english" => Ok(Language::English),
            "japanese" => Ok(Language::Japanese),
            "korean" => Ok(Language::Korean),
            "spanish" => Ok(Language::Spanish),
            "chinese_simplified" => Ok(Language::SimplifiedChinese),
            "chinese_traditional" => Ok(Language::TraditionalChinese),
            "french" => Ok(Language::French),
            "italian" => Ok(Language::Italian),
            "czech" => Ok(Language::Czech),
            "portuguese" => Ok(Language::Portuguese),
            _ => Err(ApiError::InvalidLanguage(language.to_string())),
        }
    }
}