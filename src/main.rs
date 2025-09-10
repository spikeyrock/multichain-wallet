use actix_web::{web, App, HttpServer, middleware as actix_middleware};
use actix_cors::Cors;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod api;
mod chains;
mod core;
mod errors;
mod services;
mod middleware;
mod grpc;

use api::handlers;
use services::wallet::WalletService;
use middleware::auth::ApiKeyAuth;
use grpc::{HealthServiceImpl, MnemonicServiceImpl, WalletServiceImpl};
use grpc::wallet_proto::{
    health_service_server::HealthServiceServer,
    mnemonic_service_server::MnemonicServiceServer,
    wallet_service_server::WalletServiceServer,
};
use tonic::transport::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file if present
    dotenv::dotenv().ok();
    
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // Ensure API_KEY is set
    if std::env::var("API_KEY").is_err() {
        panic!("API_KEY environment variable must be set");
    }
    
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let http_port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid number");
    let grpc_port: u16 = std::env::var("GRPC_PORT")
        .unwrap_or_else(|_| "9090".to_string())
        .parse()
        .expect("GRPC_PORT must be a valid number");
    
    info!("Starting Multichain Wallet API:");
    info!("  HTTP server on {}:{}", host, http_port);
    info!("  gRPC server on {}:{}", host, grpc_port);
    
    // Create shared wallet service
    let wallet_service = Arc::new(Mutex::new(WalletService::new()));
    
    // Clone for gRPC services
    let grpc_wallet_service = wallet_service.clone();
    
    // Create gRPC services
    let health_service = HealthServiceImpl::default();
    let mnemonic_service = MnemonicServiceImpl::new(grpc_wallet_service.clone());
    let wallet_service_grpc = WalletServiceImpl::new(grpc_wallet_service);
    
    // Prepare gRPC server
    let grpc_addr = format!("{}:{}", host, grpc_port).parse()
        .expect("Invalid gRPC address");
    
    let grpc_server = Server::builder()
        .add_service(HealthServiceServer::new(health_service))
        .add_service(MnemonicServiceServer::new(mnemonic_service))
        .add_service(WalletServiceServer::new(wallet_service_grpc))
        .serve(grpc_addr);
    
    // Prepare HTTP server
    let http_addr = (host.clone(), http_port);
    let http_server = HttpServer::new(move || {
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
            )
    })
    .bind(http_addr)?
    .run();
    
    // Run both servers concurrently
    info!("Both servers starting...");
    let (http_result, grpc_result) = tokio::join!(http_server, grpc_server);
    
    // Check results
    if let Err(e) = http_result {
        eprintln!("HTTP server error: {}", e);
    }
    if let Err(e) = grpc_result {
        eprintln!("gRPC server error: {}", e);
    }
    
    Ok(())
}