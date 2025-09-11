use tonic::{Request, Response, Status};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use super::auth::check_auth;
use super::wallet_proto::{
    mnemonic_service_server::MnemonicService,
    GenerateMnemonicRequest, GenerateMnemonicResponse,
    ValidateMnemonicRequest, ValidateMnemonicResponse,
    GetSupportedLanguagesRequest, GetSupportedLanguagesResponse,
    LanguageInfo,
};
use crate::services::wallet::WalletService;

pub struct MnemonicServiceImpl {
    wallet_service: Arc<Mutex<WalletService>>,
}

impl MnemonicServiceImpl {
    pub fn new(wallet_service: Arc<Mutex<WalletService>>) -> Self {
        Self { wallet_service }
    }
    
    fn validate_request(req: &GenerateMnemonicRequest) -> Result<(), Status> {
        // Validate language
        let valid_languages = vec![
            "english", "japanese", "korean", "spanish", 
            "chinese_simplified", "chinese_traditional", 
            "french", "italian", "czech", "portuguese"
        ];
        
        if !valid_languages.contains(&req.language.to_lowercase().as_str()) {
            return Err(Status::invalid_argument(format!("Invalid language: {}", req.language)));
        }

        // Validate word count
        match req.word_count {
            12 | 15 | 18 | 21 | 24 => Ok(()),
            _ => Err(Status::invalid_argument(format!("Invalid word count: {}. Must be 12, 15, 18, 21, or 24", req.word_count)))
        }
    }
}

#[tonic::async_trait]
impl MnemonicService for MnemonicServiceImpl {
    async fn generate_mnemonic(
        &self,
        request: Request<GenerateMnemonicRequest>,
    ) -> Result<Response<GenerateMnemonicResponse>, Status> {
        check_auth(&request)?;
        let req = request.into_inner();
        
        // Validate request
        Self::validate_request(&req)?;

        info!(
            "Generating {} word mnemonic in {}",
            req.word_count, req.language
        );

        // Generate mnemonic
        let service = self.wallet_service.lock().await;
        let mnemonic = service
            .generate_mnemonic(&req.language, req.word_count)
            .await
            .map_err(|e| Status::internal(format!("Failed to generate mnemonic: {}", e)))?;

        let response = GenerateMnemonicResponse {
            mnemonic,
            language: req.language,
            word_count: req.word_count,
            generated_at: chrono::Utc::now().timestamp(),
        };

        Ok(Response::new(response))
    }

    async fn validate_mnemonic(
        &self,
        request: Request<ValidateMnemonicRequest>,
    ) -> Result<Response<ValidateMnemonicResponse>, Status> {
        check_auth(&request)?;
        let req = request.into_inner();
        
        info!("Validating mnemonic in {}", req.language);

        let service = self.wallet_service.lock().await;
        let (valid, word_count) = service
            .validate_mnemonic(&req.mnemonic, &req.language)
            .await;

        let response = ValidateMnemonicResponse {
            valid,
            word_count: if valid { word_count } else { 0 },
            message: if valid {
                "Valid mnemonic phrase".to_string()
            } else {
                "Invalid mnemonic phrase".to_string()
            },
        };

        Ok(Response::new(response))
    }

    async fn get_supported_languages(
        &self,
        request: Request<GetSupportedLanguagesRequest>,
    ) -> Result<Response<GetSupportedLanguagesResponse>, Status> {
        check_auth(&request)?;
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

        let response = GetSupportedLanguagesResponse { languages };
        Ok(Response::new(response))
    }
}