// src/main.rs - Update your main.rs to use the middleware
use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use std::sync::{Arc, Mutex};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod api;
mod chains;
mod config;
mod core;
mod errors;
mod services;
mod middleware; // Add this line

use api::handlers;
use config::Config;
use services::wallet::WalletService;
use middleware::auth::ApiKeyAuth; // Add this line

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // Load configuration
    let config = Config::from_env();
    
    // Ensure API_KEY is set
    if std::env::var("API_KEY").is_err() {
        panic!("API_KEY environment variable must be set");
    }
    
    info!("Starting Multichain Wallet API on {}:{}", config.host, config.port);
    
    // Create shared wallet service
    let wallet_service = Arc::new(Mutex::new(WalletService::new()));
    
    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);
            
        App::new()
            .app_data(web::Data::new(wallet_service.clone()))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(ApiKeyAuth) // Add API key authentication middleware
            .service(
                web::scope("/api/v1")
                    .service(handlers::health_check)
                    .service(handlers::generate_mnemonic)
                    .service(handlers::validate_mnemonic)
                    .service(handlers::get_supported_languages)
                    .service(handlers::generate_wallet)
                    .service(handlers::batch_generate_wallets)
                    .service(handlers::get_wallet_types)
            )
    })
    .bind((config.host, config.port))?
    .run()
    .await
}