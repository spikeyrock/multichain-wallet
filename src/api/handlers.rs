use actix_web::{get, post, web, HttpResponse};
use std::sync::Arc;
use tracing::info;

use crate::api::models::*;
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
    wallet_service: web::Data<Arc<WalletService>>,
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
    let mnemonic = wallet_service
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
    wallet_service: web::Data<Arc<WalletService>>,
    req: web::Json<ValidateMnemonicRequest>,
) -> ApiResult<HttpResponse> {
    info!("Validating mnemonic in {}", req.language);

    let (valid, word_count) = wallet_service
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