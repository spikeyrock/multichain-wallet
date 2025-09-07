// src/main.rs - Fixed version
use actix_web::{web, App, HttpServer, middleware as actix_middleware};
use actix_cors::Cors;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod api;
mod chains;
mod config;
mod core;
mod errors;
mod services;
mod middleware;

use api::{handlers, tokens};
use config::Config;
use services::wallet::WalletService;
use middleware::auth::ApiKeyAuth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenv::dotenv().ok();
    
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    let host = config.host.clone();
    let port = config.port;
    
    // Ensure API_KEY is set
    if std::env::var("API_KEY").is_err() {
        panic!("API_KEY environment variable must be set");
    }
    
    info!("Starting Multichain Wallet API on {}:{}", host, port);
    
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
            .wrap(actix_middleware::Logger::default())
            .wrap(ApiKeyAuth) // Add API key authentication middleware
            .service(
                web::scope("/api/v1")
                    .service(handlers::health_check)
                    .service(handlers::generate_mnemonic)
                    .service(handlers::validate_mnemonic)
                    .service(handlers::get_supported_languages)
                    .service(handlers::generate_wallet)
                    .service(handlers::batch_generate_wallets)
                    .service(handlers::get_supported_wallet_types)
                    .configure(tokens::configure_routes)
            )
    })
    .bind((host, port))?
    .run()
    .await
}