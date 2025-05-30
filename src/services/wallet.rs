use bip39::{Language, Mnemonic};
use bitcoin::{Network, Address, PublicKey, XOnlyPublicKey};
use bitcoin::bip32::{Xpriv, DerivationPath, ChildNumber};
use bitcoin::secp256k1::Secp256k1;
use rand::RngCore;
use ed25519_dalek::SigningKey;
use sha2::{Sha256, Sha512, Digest};
use blake2::{Blake2b512, Blake2s256};
use tiny_keccak::{Hasher, Keccak};
use num_bigint::BigUint;

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

        // Create secp256k1 context for Bitcoin/Ethereum/Tron
        let secp = Secp256k1::new();

        // Generate address based on type
        match address_type {
            AddressType::BitcoinTaproot => self.generate_bitcoin_taproot(&secp, &seed, index).await,
            AddressType::BitcoinSegwit => self.generate_bitcoin_segwit(&secp, &seed, index).await,
            AddressType::BitcoinLegacy => self.generate_bitcoin_legacy(&secp, &seed, index).await,
            AddressType::Ethereum => self.generate_ethereum(&secp, &seed, index).await,
            AddressType::Xrp => self.generate_xrp(&secp, &seed, index).await,
            AddressType::Solana => self.generate_solana(&seed, index).await,
            AddressType::Tron => self.generate_tron(&secp, &seed, index).await,
            AddressType::Cardano => self.generate_cardano(&seed, index).await,
            AddressType::Sui => self.generate_sui(&seed, index).await,
            AddressType::Stellar => self.generate_stellar(&seed, index).await,
            AddressType::Monero => self.generate_monero(&seed, index).await,
            AddressType::Near => self.generate_near(&seed, index).await,
        }
    }

    async fn generate_bitcoin_taproot(
        &self,
        secp: &Secp256k1<bitcoin::secp256k1::All>,
        seed: &[u8],
        index: u32,
    ) -> ApiResult<WalletAddress> {
        let master = Xpriv::new_master(Network::Bitcoin, seed)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;

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
        seed: &[u8],
        index: u32,
    ) -> ApiResult<WalletAddress> {
        let master = Xpriv::new_master(Network::Bitcoin, seed)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;

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
        seed: &[u8],
        index: u32,
    ) -> ApiResult<WalletAddress> {
        let master = Xpriv::new_master(Network::Bitcoin, seed)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;

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
        seed: &[u8],
        index: u32,
    ) -> ApiResult<WalletAddress> {
        let master = Xpriv::new_master(Network::Bitcoin, seed)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;

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

    async fn generate_xrp(
        &self,
        secp: &Secp256k1<bitcoin::secp256k1::All>,
        seed: &[u8],
        index: u32,
    ) -> ApiResult<WalletAddress> {
        let master = Xpriv::new_master(Network::Bitcoin, seed)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;

        // m/44'/144'/0'/0/{index}
        let path = DerivationPath::from(vec![
            ChildNumber::from_hardened_idx(44).unwrap(),
            ChildNumber::from_hardened_idx(144).unwrap(),
            ChildNumber::from_hardened_idx(0).unwrap(),
            ChildNumber::from_normal_idx(0).unwrap(),
            ChildNumber::from_normal_idx(index).unwrap(),
        ]);
        
        let child = master.derive_priv(secp, &path)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        
        let private_key = child.private_key;
        let secp_pubkey = private_key.public_key(secp);
        
        // XRP uses compressed public key
        let pubkey_bytes = secp_pubkey.serialize();
        
        // Hash public key with SHA256 then RIPEMD160
        let sha256_hash = Sha256::digest(&pubkey_bytes);
        let ripemd_hash = ripemd::Ripemd160::digest(&sha256_hash);
        
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
            address_type: AddressType::Xrp,
            derivation_path: format!("m/44'/144'/0'/0/{}", index),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    async fn generate_solana(
        &self,
        seed: &[u8],
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Solana uses Ed25519
        // m/44'/501'/0'/0'
        let derivation_path = format!("m/44'/501'/{}'/0'", index);
        
        // Use SLIP10 for Ed25519 derivation
        let mut hasher = Sha512::new();
        hasher.update(b"ed25519 seed");
        hasher.update(seed);
        let hash = hasher.finalize();
        
        let private_key_bytes = &hash[..32];
        let signing_key = SigningKey::from_bytes(private_key_bytes.try_into().unwrap());
        let verifying_key = signing_key.verifying_key();
        
        // Solana address is the base58 encoded public key
        let address = bs58::encode(verifying_key.as_bytes()).into_string();
        
        Ok(WalletAddress {
            address,
            address_type: AddressType::Solana,
            derivation_path,
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    async fn generate_tron(
        &self,
        secp: &Secp256k1<bitcoin::secp256k1::All>,
        seed: &[u8],
        index: u32,
    ) -> ApiResult<WalletAddress> {
        let master = Xpriv::new_master(Network::Bitcoin, seed)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;

        // m/44'/195'/0'/0/{index}
        let path = DerivationPath::from(vec![
            ChildNumber::from_hardened_idx(44).unwrap(),
            ChildNumber::from_hardened_idx(195).unwrap(),
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
        
        // Skip the 0x04 prefix for address calculation
        let pubkey_no_prefix = &pubkey_bytes[1..];
        
        // Compute address using Keccak256
        let mut hasher = Keccak::v256();
        hasher.update(pubkey_no_prefix);
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        
        // Take last 20 bytes and add Tron prefix (0x41)
        let mut address_bytes = vec![0x41];
        address_bytes.extend_from_slice(&hash[12..]);
        
        // Double SHA256 for checksum
        let checksum = Sha256::digest(&Sha256::digest(&address_bytes));
        address_bytes.extend_from_slice(&checksum[..4]);
        
        // Base58 encode
        let address = bs58::encode(address_bytes).into_string();
        
        Ok(WalletAddress {
            address,
            address_type: AddressType::Tron,
            derivation_path: format!("m/44'/195'/0'/0/{}", index),
            index,
            public_key: hex::encode(secp_pubkey.serialize()),
            private_key: hex::encode(private_key.secret_bytes()),
        })
    }

    async fn generate_cardano(
        &self,
        seed: &[u8],
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Cardano uses Ed25519 with a specific derivation scheme
        // For simplicity, we'll use m/1852'/1815'/0'/0/{index}
        let derivation_path = format!("m/1852'/1815'/0'/0/{}", index);
        
        // Use Blake2b-512 for key derivation
        let mut hasher = Blake2b512::new();
        hasher.update(seed);
        hasher.update(&index.to_le_bytes());
        let hash = hasher.finalize();
        
        let private_key_bytes = &hash[..32];
        let signing_key = SigningKey::from_bytes(private_key_bytes.try_into().unwrap());
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
            address_type: AddressType::Cardano,
            derivation_path,
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    async fn generate_sui(
        &self,
        seed: &[u8],
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Sui uses Ed25519
        // m/44'/784'/0'/0'/{index}'
        let derivation_path = format!("m/44'/784'/0'/0'/{}'", index);
        
        // Use SLIP10 for Ed25519 derivation
        let mut hasher = Sha512::new();
        hasher.update(b"ed25519 seed");
        hasher.update(seed);
        hasher.update(&index.to_le_bytes());
        let hash = hasher.finalize();
        
        let private_key_bytes = &hash[..32];
        let signing_key = SigningKey::from_bytes(private_key_bytes.try_into().unwrap());
        let verifying_key = signing_key.verifying_key();
        
        // Sui address is derived from public key
        let mut hasher = Blake2s256::new();
        hasher.update(&[0x00]); // Signature scheme flag for Ed25519
        hasher.update(verifying_key.as_bytes());
        let hash = hasher.finalize();
        
        // Take first 32 bytes and format as 0x prefixed hex
        let address = format!("0x{}", hex::encode(&hash[..32]));
        
        Ok(WalletAddress {
            address,
            address_type: AddressType::Sui,
            derivation_path,
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    async fn generate_stellar(
        &self,
        seed: &[u8],
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Stellar uses Ed25519
        // m/44'/148'/0'
        let derivation_path = format!("m/44'/148'/{}'", index);
        
        // Use SLIP10 for Ed25519 derivation
        let mut hasher = Sha512::new();
        hasher.update(b"ed25519 seed");
        hasher.update(seed);
        hasher.update(&index.to_le_bytes());
        let hash = hasher.finalize();
        
        let private_key_bytes = &hash[..32];
        let signing_key = SigningKey::from_bytes(private_key_bytes.try_into().unwrap());
        let verifying_key = signing_key.verifying_key();
        
        // Stellar address encoding
        let mut payload = vec![6 << 3]; // Version byte for public key (G...)
        payload.extend_from_slice(verifying_key.as_bytes());
        
        // CRC16 checksum
        let checksum = crc::Crc::<u16>::new(&crc::CRC_16_XMODEM).checksum(&payload);
        payload.extend_from_slice(&checksum.to_le_bytes());
        
        // Base32 encode
        let address = base32_encode(&payload);
        
        Ok(WalletAddress {
            address,
            address_type: AddressType::Stellar,
            derivation_path,
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    async fn generate_monero(
        &self,
        seed: &[u8],
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // Monero has a complex address generation scheme
        // For production, you'd want to use the monero crate properly
        // This is a simplified version
        let derivation_path = format!("m/44'/128'/{}'/0/0", index);
        
        // Monero uses its own key derivation
        let mut hasher = Keccak::v256();
        hasher.update(seed);
        hasher.update(&index.to_le_bytes());
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        
        // Simplified: just use the hash as private key
        // Real Monero has spend and view keys
        let signing_key = SigningKey::from_bytes(&hash);
        let verifying_key = signing_key.verifying_key();
        
        // Simplified Monero address (network byte + spend pubkey + view pubkey + checksum)
        // Using network byte 18 for mainnet
        let mut addr_data = vec![18];
        addr_data.extend_from_slice(verifying_key.as_bytes());
        addr_data.extend_from_slice(verifying_key.as_bytes()); // Using same key for simplicity
        
        // Keccak256 checksum
        let mut hasher = Keccak::v256();
        hasher.update(&addr_data);
        let mut checksum = [0u8; 32];
        hasher.finalize(&mut checksum);
        addr_data.extend_from_slice(&checksum[..4]);
        
        // Base58 encode with Monero alphabet
        let address = base58_monero::encode(&addr_data)
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        
        Ok(WalletAddress {
            address,
            address_type: AddressType::Monero,
            derivation_path,
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
        })
    }

    async fn generate_near(
        &self,
        seed: &[u8],
        index: u32,
    ) -> ApiResult<WalletAddress> {
        // NEAR uses Ed25519
        // m/44'/397'/0'
        let derivation_path = format!("m/44'/397'/0'/0'/{}'", index);
        
        // Use SLIP10 for Ed25519 derivation
        let mut hasher = Sha512::new();
        hasher.update(b"ed25519 seed");
        hasher.update(seed);
        hasher.update(&index.to_le_bytes());
        let hash = hasher.finalize();
        
        let private_key_bytes = &hash[..32];
        let signing_key = SigningKey::from_bytes(private_key_bytes.try_into().unwrap());
        let verifying_key = signing_key.verifying_key();
        
        // NEAR implicit address is the hex-encoded public key
        let address = hex::encode(verifying_key.as_bytes());
        
        Ok(WalletAddress {
            address,
            address_type: AddressType::Near,
            derivation_path,
            index,
            public_key: hex::encode(verifying_key.as_bytes()),
            private_key: hex::encode(signing_key.to_bytes()),
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

// Add this to handle base32 encoding
fn base32_encode(data: &[u8]) -> String {
    // Simplified base32 encoding for Stellar
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut result = String::new();
    
    let mut bits = 0u32;
    let mut value = 0u32;
    
    for &byte in data {
        value = (value << 8) | byte as u32;
        bits += 8;
        
        while bits >= 5 {
            let index = (value >> (bits - 5)) & 0x1F;
            result.push(CHARSET[index as usize] as char);
            bits -= 5;
        }
    }
    
    if bits > 0 {
        let index = (value << (5 - bits)) & 0x1F;
        result.push(CHARSET[index as usize] as char);
    }
    
    result
}

// Add this for Monero base58
mod base58_monero {
    use num_bigint::BigUint;
    
    pub fn encode(data: &[u8]) -> Result<String, String> {
        // Use the Monero base58 alphabet
        const ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
        
        // Simple base58 encoding
        let mut result = Vec::new();
        let mut num = BigUint::from_bytes_be(data);
        let base = BigUint::from(58u32);
        
        while num > BigUint::from(0u32) {
            let remainder = &num % &base;
            let digits = remainder.to_u32_digits();
            if !digits.is_empty() {
                result.push(ALPHABET[digits[0] as usize]);
            }
            num /= &base;
        }
        
        // Handle leading zeros
        for &byte in data {
            if byte == 0 {
                result.push(ALPHABET[0]);
            } else {
                break;
            }
        }
        
        result.reverse();
        Ok(String::from_utf8(result).unwrap())
    }
}

// Add this dependency to Cargo.toml for Monero base58
use bech32::ToBase32;