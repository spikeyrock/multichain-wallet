use actix_web::{get, post, web, HttpResponse};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use crate::api::models::*;
use crate::core::{ChainType, get_chain_info};
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
    let chain_type: ChainType = req.address_type.clone().into();
    let chain_info = get_chain_info(&chain_type);
    
    info!(
        "Generating {} ({}) wallet at index {}",
        chain_info.name, chain_info.symbol, req.index
    );

    let mut service = wallet_service.lock().await;
    let wallet = service
        .generate_wallet_address(
            &req.mnemonic,
            &req.passphrase,
            &chain_type,
            req.index,
        )
        .await?;

    let response = GenerateWalletResponse {
        address: wallet.address,
        chain_name: wallet.chain_info.name,
        chain_symbol: wallet.chain_info.symbol,
        address_type: req.address_type.clone(),
        derivation_path: wallet.derivation_path,
        index: wallet.index,
        public_key: wallet.public_key,
        private_key: wallet.private_key,
    };

    Ok(HttpResponse::Ok().json(response))
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

    if req.address_types.is_empty() {
        return Err(ApiError::BadRequest(
            "At least one address type must be specified".to_string(),
        ));
    }

    info!(
        "Batch generating {} addresses for {} types starting at index {}",
        req.count,
        req.address_types.len(),
        req.start_index
    );

    let chain_types: Vec<ChainType> = req.address_types
        .iter()
        .map(|at| at.clone().into())
        .collect();

    let mut service = wallet_service.lock().await;
    let addresses = service
        .batch_generate_wallet_addresses(
            &req.mnemonic,
            &req.passphrase,
            &chain_types,
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
    
    let chain_details: Vec<serde_json::Value> = chains
        .into_iter()
        .map(|chain| {
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
                _ => "unknown"
            };
            
            serde_json::json!({
                "type": chain_type,
                "name": chain.name,
                "symbol": chain.symbol,
                "decimals": chain.decimals,
                "description": format!("{} ({}) - {} decimals", chain.name, chain.symbol, chain.decimals)
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "supported_chains": chain_details,
        "total_chains": chain_details.len()
    })))
}