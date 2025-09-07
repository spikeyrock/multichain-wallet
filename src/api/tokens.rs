use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use crate::core::{get_token_registry, UnifiedToken, ChainDeployment};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: Option<UnifiedToken>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenListResponse {
    pub tokens: Vec<UnifiedToken>,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainTokensRequest {
    pub chain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDeploymentRequest {
    pub base_symbol: String,
    pub chain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentResponse {
    pub deployment: Option<ChainDeployment>,
    pub message: String,
}

// Get all tokens
pub async fn get_all_tokens() -> Result<HttpResponse> {
    let registry = get_token_registry();
    let tokens = registry.get_all_tokens();
    
    Ok(HttpResponse::Ok().json(TokenListResponse {
        count: tokens.len(),
        tokens,
    }))
}

// Get token by base symbol
pub async fn get_token(path: web::Path<String>) -> Result<HttpResponse> {
    let base_symbol = path.into_inner();
    let registry = get_token_registry();
    
    match registry.get_token(&base_symbol) {
        Some(token) => Ok(HttpResponse::Ok().json(TokenResponse {
            token: Some(token.clone()),
            message: format!("Token {} found", base_symbol),
        })),
        None => Ok(HttpResponse::NotFound().json(TokenResponse {
            token: None,
            message: format!("Token {} not found", base_symbol),
        })),
    }
}

// Get all tokens on a specific chain
pub async fn get_tokens_by_chain(req: web::Json<ChainTokensRequest>) -> Result<HttpResponse> {
    let registry = get_token_registry();
    let tokens = registry.get_tokens_by_chain(&req.chain);
    
    Ok(HttpResponse::Ok().json(TokenListResponse {
        count: tokens.len(),
        tokens,
    }))
}

// Get all stablecoins
pub async fn get_stablecoins() -> Result<HttpResponse> {
    let registry = get_token_registry();
    let tokens = registry.get_stablecoins();
    
    Ok(HttpResponse::Ok().json(TokenListResponse {
        count: tokens.len(),
        tokens,
    }))
}

// Get all native tokens
pub async fn get_native_tokens() -> Result<HttpResponse> {
    let registry = get_token_registry();
    let tokens = registry.get_native_tokens();
    
    Ok(HttpResponse::Ok().json(TokenListResponse {
        count: tokens.len(),
        tokens,
    }))
}

// Get token variants (all chain deployments)
pub async fn get_token_variants(path: web::Path<String>) -> Result<HttpResponse> {
    let base_symbol = path.into_inner();
    let registry = get_token_registry();
    let variants = registry.get_token_variants(&base_symbol);
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "base_symbol": base_symbol,
        "deployments": variants,
        "count": variants.len(),
    })))
}

// Get specific token deployment on a chain
pub async fn get_token_deployment(req: web::Json<TokenDeploymentRequest>) -> Result<HttpResponse> {
    let registry = get_token_registry();
    
    match registry.get_token_deployment(&req.base_symbol, &req.chain) {
        Some(deployment) => Ok(HttpResponse::Ok().json(DeploymentResponse {
            deployment: Some(deployment),
            message: format!("Deployment found for {} on {}", req.base_symbol, req.chain),
        })),
        None => Ok(HttpResponse::NotFound().json(DeploymentResponse {
            deployment: None,
            message: format!("No deployment found for {} on {}", req.base_symbol, req.chain),
        })),
    }
}

// Check if token is multichain
pub async fn is_multichain(path: web::Path<String>) -> Result<HttpResponse> {
    let base_symbol = path.into_inner();
    let registry = get_token_registry();
    let is_multichain = registry.is_multi_chain_token(&base_symbol);
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "base_symbol": base_symbol,
        "is_multichain": is_multichain,
    })))
}

// Search tokens by symbol (supports partial matches)
pub async fn search_tokens(query: web::Path<String>) -> Result<HttpResponse> {
    let search_term = query.into_inner().to_lowercase();
    let registry = get_token_registry();
    
    let tokens: Vec<UnifiedToken> = registry
        .get_all_tokens()
        .into_iter()
        .filter(|token| {
            token.base_symbol.to_lowercase().contains(&search_term) ||
            token.name.to_lowercase().contains(&search_term) ||
            token.deployments.iter().any(|d| d.symbol.to_lowercase().contains(&search_term))
        })
        .collect();
    
    Ok(HttpResponse::Ok().json(TokenListResponse {
        count: tokens.len(),
        tokens,
    }))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tokens")
            .route("/all", web::get().to(get_all_tokens))
            .route("/token/{symbol}", web::get().to(get_token))
            .route("/by-chain", web::post().to(get_tokens_by_chain))
            .route("/stablecoins", web::get().to(get_stablecoins))
            .route("/native", web::get().to(get_native_tokens))
            .route("/variants/{symbol}", web::get().to(get_token_variants))
            .route("/deployment", web::post().to(get_token_deployment))
            .route("/multichain/{symbol}", web::get().to(is_multichain))
            .route("/search/{query}", web::get().to(search_tokens))
    );
}