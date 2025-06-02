use actix_web::{get, post, web, HttpResponse};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use crate::api::models::*;
use crate::core::{get_chain_info, get_chain_types_by_symbol};
use crate::errors::{ApiError, ApiResult};
use crate::services::wallet::WalletService;

#[get("/health")]
pub async fn health_check() -> ApiResult<HttpResponse> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now().timestamp(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[post("/mnemonic/generate")]
pub async fn generate_mnemonic(
    wallet_service: web::Data<Arc<Mutex<WalletService>>>,
    req: web::Json<GenerateMnemonicRequest>,
) -> ApiResult<HttpResponse> {
    // Validate request
    req.validate()
        .map_err(|e| ApiError::BadRequest(e))?;

    info!(
        "Generating {} word mnemonic in {}",
        req.word_count, req.language
    );

    // Generate mnemonic
    let service = wallet_service.lock().await;
    let mnemonic = service
        .generate_mnemonic(&req.language, req.word_count)
        .await?;

    let response = GenerateMnemonicResponse {
        mnemonic,
        language: req.language.clone(),
        word_count: req.word_count,
        generated_at: chrono::Utc::now().timestamp(),
    };

    Ok(HttpResponse::Ok().json(response))
}

#[post("/mnemonic/validate")]
pub async fn validate_mnemonic(
    wallet_service: web::Data<Arc<Mutex<WalletService>>>,
    req: web::Json<ValidateMnemonicRequest>,
) -> ApiResult<HttpResponse> {
    info!("Validating mnemonic in {}", req.language);

    let service = wallet_service.lock().await;
    let (valid, word_count) = service
        .validate_mnemonic(&req.mnemonic, &req.language)
        .await;

    let response = ValidateMnemonicResponse {
        valid,
        word_count: if valid { Some(word_count) } else { None },
        message: if valid {
            Some("Valid mnemonic phrase".to_string())
        } else {
            Some("Invalid mnemonic phrase".to_string())
        },
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/languages")]
pub async fn get_supported_languages() -> ApiResult<HttpResponse> {
    let languages = vec![
        LanguageInfo {
            code: "english".to_string(),
            name: "English".to_string(),
            native_name: "English".to_string(),
        },
        LanguageInfo {
            code: "japanese".to_string(),
            name: "Japanese".to_string(),
            native_name: "日本語".to_string(),
        },
        LanguageInfo {
            code: "korean".to_string(),
            name: "Korean".to_string(),
            native_name: "한국어".to_string(),
        },
        LanguageInfo {
            code: "spanish".to_string(),
            name: "Spanish".to_string(),
            native_name: "Español".to_string(),
        },
        LanguageInfo {
            code: "chinese_simplified".to_string(),
            name: "Chinese (Simplified)".to_string(),
            native_name: "中文(简体)".to_string(),
        },
        LanguageInfo {
            code: "chinese_traditional".to_string(),
            name: "Chinese (Traditional)".to_string(),
            native_name: "中文(繁體)".to_string(),
        },
        LanguageInfo {
            code: "french".to_string(),
            name: "French".to_string(),
            native_name: "Français".to_string(),
        },
        LanguageInfo {
            code: "italian".to_string(),
            name: "Italian".to_string(),
            native_name: "Italiano".to_string(),
        },
        LanguageInfo {
            code: "czech".to_string(),
            name: "Czech".to_string(),
            native_name: "Čeština".to_string(),
        },
        LanguageInfo {
            code: "portuguese".to_string(),
            name: "Portuguese".to_string(),
            native_name: "Português".to_string(),
        },
    ];

    let response = SupportedLanguagesResponse { languages };

    Ok(HttpResponse::Ok().json(response))
}

#[post("/wallet/generate")]
pub async fn generate_wallet(
    wallet_service: web::Data<Arc<Mutex<WalletService>>>,
    req: web::Json<GenerateWalletRequest>,
) -> ApiResult<HttpResponse> {
    // Get chain types for the symbol
    let chain_types = get_chain_types_by_symbol(&req.symbol);
    
    if chain_types.is_empty() {
        return Err(ApiError::BadRequest(
            format!("Unsupported symbol: {}", req.symbol)
        ));
    }
    
    let mut service = wallet_service.lock().await;
    let mut wallets = Vec::new();
    
    // Generate wallets for all chain types associated with the symbol
    for chain_type in chain_types {
        let chain_info = get_chain_info(&chain_type);
        
        info!(
            "Generating {} ({}) wallet at index {}",
            chain_info.name, chain_info.symbol, req.index
        );
        
        let wallet = service
            .generate_wallet_address(
                &req.mnemonic,
                &req.passphrase,
                &chain_type,
                req.index,
            )
            .await?;
            
        wallets.push(wallet);
    }
    
    // If only one wallet (most coins), return single response
    if wallets.len() == 1 {
        let wallet = wallets.into_iter().next().unwrap();
        let response = GenerateWalletResponse {
            address: wallet.address,
            chain_name: wallet.chain_info.name,
            chain_symbol: wallet.chain_info.symbol,
            address_type: wallet.chain_type.into(),
            derivation_path: wallet.derivation_path,
            index: wallet.index,
            public_key: wallet.public_key,
            private_key: wallet.private_key,
        };
        Ok(HttpResponse::Ok().json(response))
    } else {
        // For multiple wallets (BTC), return array
        let responses: Vec<GenerateWalletResponse> = wallets
            .into_iter()
            .map(|wallet| GenerateWalletResponse {
                address: wallet.address,
                chain_name: wallet.chain_info.name,
                chain_symbol: wallet.chain_info.symbol,
                address_type: wallet.chain_type.into(),
                derivation_path: wallet.derivation_path,
                index: wallet.index,
                public_key: wallet.public_key,
                private_key: wallet.private_key,
            })
            .collect();
        Ok(HttpResponse::Ok().json(responses))
    }
}

#[post("/wallet/batch")]
pub async fn batch_generate_wallets(
    wallet_service: web::Data<Arc<Mutex<WalletService>>>,
    req: web::Json<BatchGenerateWalletRequest>,
) -> ApiResult<HttpResponse> {
    // Validate count
    if req.count == 0 || req.count > 100 {
        return Err(ApiError::BadRequest(
            "Count must be between 1 and 100".to_string(),
        ));
    }

    if req.symbols.is_empty() {
        return Err(ApiError::BadRequest(
            "At least one symbol must be specified".to_string(),
        ));
    }

    // Convert symbols to chain types
    let mut all_chain_types = Vec::new();
    for symbol in &req.symbols {
        let chain_types = get_chain_types_by_symbol(symbol);
        if chain_types.is_empty() {
            return Err(ApiError::BadRequest(
                format!("Unsupported symbol: {}", symbol)
            ));
        }
        all_chain_types.extend(chain_types);
    }

    info!(
        "Batch generating {} addresses for {} chain types starting at index {}",
        req.count,
        all_chain_types.len(),
        req.start_index
    );

    let mut service = wallet_service.lock().await;
    let addresses = service
        .batch_generate_wallet_addresses(
            &req.mnemonic,
            &req.passphrase,
            &all_chain_types,
            req.start_index,
            req.count,
        )
        .await?;

    let response_addresses: Vec<WalletAddressResponse> = addresses
        .into_iter()
        .map(|wallet| WalletAddressResponse {
            address: wallet.address,
            chain_name: wallet.chain_info.name,
            chain_symbol: wallet.chain_info.symbol,
            address_type: wallet.chain_type.into(),
            derivation_path: wallet.derivation_path,
            index: wallet.index,
            public_key: wallet.public_key,
            private_key: wallet.private_key,
        })
        .collect();

    let response = BatchGenerateWalletResponse {
        addresses: response_addresses,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/wallet/types")]
pub async fn get_supported_wallet_types(
    wallet_service: web::Data<Arc<Mutex<WalletService>>>,
) -> ApiResult<HttpResponse> {
    let service = wallet_service.lock().await;
    let chains = service.list_supported_chains().await;
    
    // Group chains by symbol
    let mut symbols_map: std::collections::HashMap<String, Vec<serde_json::Value>> = std::collections::HashMap::new();
    
    let total_chains = chains.len();
    
    for chain in chains {
        let chain_type = match chain.name.as_str() {
            "Bitcoin" => {
                // Determine Bitcoin variant based on address format
                match &chain.address_format {
                    crate::core::AddressFormat::Bech32 { hrp } if hrp == "bc" => "bitcoin_segwit",
                    crate::core::AddressFormat::Bitcoin { .. } => "bitcoin_legacy",
                    _ => "bitcoin_taproot",
                }
            },
            "Ethereum" => "ethereum",
            "Ripple" => "xrp",
            "Solana" => "solana",
            "TRON" => "tron",
            "Cardano" => "cardano",
            "Sui" => "sui",
            "Stellar" => "stellar",
            "Monero" => "monero",
            "NEAR Protocol" => "near",
            "Toncoin" => "ton",
            "Dogecoin" => "dogecoin",
            "Polkadot" => "polkadot",
            "Cosmos" => "cosmos",
            "Osmosis" => "osmosis",
            "Juno" => "juno",
            "Secret Network" => "secret",
            "Akash" => "akash",
            "Sei" => "sei",
            "Celestia" => "celestia",
            "Injective" => "injective",
            "Tezos" => "tezos",
            "Algorand" => "algorand",
            "EOS" => "eos",
            "Hedera" => "hedera",
            "Filecoin" => "filecoin",
            "Mina" => "mina",
            "Internet Computer" => "internet_computer",
            _ => "unknown"
        };
        
        let chain_detail = serde_json::json!({
            "type": chain_type,
            "name": chain.name,
            "symbol": chain.symbol,
            "decimals": chain.decimals,
            "description": format!("{} ({}) - {} decimals", chain.name, chain.symbol, chain.decimals)
        });
        
        symbols_map.entry(chain.symbol.clone())
            .or_insert_with(Vec::new)
            .push(chain_detail);
    }
    
    // Get list of supported symbols
    let supported_symbols: Vec<&str> = vec![
        "BTC", "ETH", "XRP", "SOL", "TRX", "ADA", "SUI", "XLM", "XMR", "NEAR", "TON", "DOGE", "DOT",
        "ATOM", "OSMO", "JUNO", "SCRT", "AKT", "SEI", "TIA", "INJ", "XTZ", "ALGO", "EOS", "HBAR", "FIL", "MINA", "ICP"
    ];
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "supported_symbols": supported_symbols,
        "chains_by_symbol": symbols_map,
        "total_chains": total_chains
    })))
}


#[get("/test/solana/all")]
pub async fn test_all_solana_methods() -> ApiResult<HttpResponse> {
    use bip39::Mnemonic;
    use sha2::{Sha512, Digest};
    use hmac::{Hmac, Mac};
    use ed25519_dalek::SigningKey;
    
    let mnemonic_str = "satoshi scheme wasp there brick client warm neutral joy pelican absent seven earth clog bus dizzy west fruit focus jacket demise juice fish mushroom";
    let expected = "ErD4UTnfDXEMkruTTFPKR3sbikboT3RhNMAKwcocH8gW";
    
    let mnemonic = Mnemonic::parse(mnemonic_str).unwrap();
    let seed = mnemonic.to_seed("");
    
    // Test different derivation paths
    let paths: Vec<(&str, Vec<u32>)> = vec![
        ("m/44'/501'", vec![0x80000044, 0x800001f5]),
        ("m/44'/501'/0'", vec![0x80000044, 0x800001f5, 0x80000000]),
        ("m/44'/501'/0'/0", vec![0x80000044, 0x800001f5, 0x80000000, 0x00000000]),
        ("m/44'/501'/0'/0'", vec![0x80000044, 0x800001f5, 0x80000000, 0x80000000]),
        ("m/44'/501'/0'/0/0", vec![0x80000044, 0x800001f5, 0x80000000, 0x00000000, 0x00000000]),
        ("m/501'/0'/0/0", vec![0x800001f5, 0x80000000, 0x00000000, 0x00000000]),
        ("m/501'", vec![0x800001f5]),
    ];
    
    let mut results = vec![];
    
    for (path_str, indices) in paths {
        type HmacSha512 = Hmac<Sha512>;
        
        let mut mac = HmacSha512::new_from_slice(b"ed25519 seed")
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        mac.update(&seed);
        let master = mac.finalize().into_bytes();
        
        let mut key = master[..32].to_vec();
        let mut chain_code = master[32..].to_vec();
        
        for &index in &indices {
            let mut mac = HmacSha512::new_from_slice(&chain_code)
                .map_err(|e| ApiError::CryptoError(e.to_string()))?;
            
            if index & 0x80000000 != 0 {
                // Hardened
                mac.update(&[0x00]);
                mac.update(&key);
            } else {
                // Non-hardened - need public key
                let temp_key = SigningKey::from_bytes(&key.as_slice().try_into().unwrap());
                mac.update(&[0x00]);
                mac.update(temp_key.verifying_key().as_bytes());
            }
            mac.update(&index.to_be_bytes());
            
            let result = mac.finalize().into_bytes();
            key = result[..32].to_vec();
            chain_code = result[32..].to_vec();
        }
        
        let signing_key = SigningKey::from_bytes(&key.as_slice().try_into().unwrap());
        let address = bs58::encode(signing_key.verifying_key().as_bytes()).into_string();
        
        results.push(serde_json::json!({
            "path": path_str,
            "address": address,
            "matches": address == expected,
            "private_key": hex::encode(&key)
        }));
        
        println!("{}: {} {}", path_str, address, if address == expected { "✓" } else { "" });
    }
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "expected": expected,
        "results": results
    })))
}



#[get("/test/solana/bip32")]
pub async fn test_solana_bip32_ed25519() -> ApiResult<HttpResponse> {
    use bip39::Mnemonic;
    use sha2::{Sha512, Sha256, Digest};
    use hmac::{Hmac, Mac};
    use ed25519_dalek::SigningKey;
    
    let mnemonic_str = "satoshi scheme wasp there brick client warm neutral joy pelican absent seven earth clog bus dizzy west fruit focus jacket demise juice fish mushroom";
    let expected = "ErD4UTnfDXEMkruTTFPKR3sbikboT3RhNMAKwcocH8gW";
    
    let mnemonic = Mnemonic::parse(mnemonic_str).unwrap();
    let seed = mnemonic.to_seed("");
    
    println!("\n=== TESTING BIP32-Ed25519 VARIANTS ===");
    println!("Seed: {}", hex::encode(&seed));
    
    let mut results = vec![];
    
    // Test 1: Direct seed to Ed25519 (no derivation)
    {
        let signing_key = SigningKey::from_bytes(&seed[..32].try_into().unwrap());
        let address = bs58::encode(signing_key.verifying_key().as_bytes()).into_string();
        results.push(serde_json::json!({
            "method": "Direct seed[0:32]",
            "address": address,
            "matches": address == expected,
        }));
    }
    
    // Test 2: SHA512 of seed, then Ed25519
    {
        let mut hasher = Sha512::new();
        hasher.update(&seed);
        let hash = hasher.finalize();
        let signing_key = SigningKey::from_bytes(&hash[..32].try_into().unwrap());
        let address = bs58::encode(signing_key.verifying_key().as_bytes()).into_string();
        results.push(serde_json::json!({
            "method": "SHA512(seed)[0:32]",
            "address": address,
            "matches": address == expected,
        }));
    }
    
    // Test 3: PBKDF2 style derivation
    {
        let mut hasher = Sha512::new();
        hasher.update(b"mnemonic");
        hasher.update(&seed);
        let hash = hasher.finalize();
        let signing_key = SigningKey::from_bytes(&hash[..32].try_into().unwrap());
        let address = bs58::encode(signing_key.verifying_key().as_bytes()).into_string();
        results.push(serde_json::json!({
            "method": "SHA512('mnemonic' + seed)[0:32]",
            "address": address,
            "matches": address == expected,
        }));
    }
    
    // Test 4: BIP32-Ed25519 with different salt
    {
        type HmacSha512 = Hmac<Sha512>;
        let mut mac = HmacSha512::new_from_slice(b"BIP0032seed") // Different salt
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        mac.update(&seed);
        let master = mac.finalize().into_bytes();
        
        let signing_key = SigningKey::from_bytes(&master[..32].try_into().unwrap());
        let address = bs58::encode(signing_key.verifying_key().as_bytes()).into_string();
        results.push(serde_json::json!({
            "method": "HMAC-SHA512('BIP0032seed', seed)[0:32]",
            "address": address,
            "matches": address == expected,
        }));
    }
    
    // Test 5: Cardano-style derivation (they use Ed25519 differently)
    {
        type HmacSha512 = Hmac<Sha512>;
        let mut mac = HmacSha512::new_from_slice(&seed[0..32])
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        mac.update(&[1u8]); // Cardano uses a counter
        let result = mac.finalize().into_bytes();
        
        let signing_key = SigningKey::from_bytes(&result[..32].try_into().unwrap());
        let address = bs58::encode(signing_key.verifying_key().as_bytes()).into_string();
        results.push(serde_json::json!({
            "method": "HMAC-SHA512(seed[0:32], 0x01)[0:32]",
            "address": address,
            "matches": address == expected,
        }));
    }
    
    // Test 6: Trust Wallet specific - they might use coin type in the salt
    {
        type HmacSha512 = Hmac<Sha512>;
        let mut mac = HmacSha512::new_from_slice(b"ed25519 seed")
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        mac.update(&seed);
        let master = mac.finalize().into_bytes();
        
        // Derive m/44'/501' but with different child key derivation
        let mut key = master[..32].to_vec();
        
        // Try deriving with just the coin type
        let mut mac2 = HmacSha512::new_from_slice(b"Solana seed")
            .map_err(|e| ApiError::CryptoError(e.to_string()))?;
        mac2.update(&key);
        let derived = mac2.finalize().into_bytes();
        
        let signing_key = SigningKey::from_bytes(&derived[..32].try_into().unwrap());
        let address = bs58::encode(signing_key.verifying_key().as_bytes()).into_string();
        results.push(serde_json::json!({
            "method": "Custom Trust Wallet method",
            "address": address,
            "matches": address == expected,
        }));
    }
    
    // Print all results
    for result in &results {
        println!("{}: {} {}", 
            result["method"], 
            result["address"],
            if result["matches"] == true { "✓" } else { "" }
        );
    }
    
    // If none match, let's also show what the expected private key might be
    // by reverse engineering from the public key (if we had it)
    let expected_pubkey = bs58::decode(expected).into_vec().unwrap();
    println!("\nExpected public key (hex): {}", hex::encode(&expected_pubkey));
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "expected": expected,
        "expected_pubkey_hex": hex::encode(&expected_pubkey),
        "results": results,
        "note": "If none match, Trust Wallet might be using a proprietary derivation method"
    })))
}