use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod api;
mod chains;
mod config;
mod core;
mod errors;
mod services;

use api::handlers;
use config::Config;
use services::wallet::WalletService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    let bind_address = format!("{}:{}", config.host, config.port);

    info!("Starting Multi-Chain Crypto Wallet API on {}", bind_address);
    info!("Supported chains: Bitcoin (BTC), Ethereum (ETH), Ripple (XRP), Solana (SOL), TRON (TRX), Cardano (ADA), Sui (SUI), Stellar (XLM), Monero (XMR), NEAR Protocol (NEAR), Toncoin (TON), Dogecoin (DOGE), Polkadot (DOT), Cosmos (ATOM), Osmosis (OSMO), Juno (JUNO), Secret (SCRT), Akash (AKT), Sei (SEI), Celestia (TIA), Injective (INJ), Tezos (XTZ), Algorand (ALGO), EOS (EOS), Hedera (HBAR), Filecoin (FIL), Mina (MINA), Internet Computer (ICP)");

    // Create shared services
    let wallet_service = Arc::new(tokio::sync::Mutex::new(WalletService::new()));

    // Start HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            // Add services to app data
            .app_data(web::Data::new(wallet_service.clone()))
            // Add middlewares
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(tracing_actix_web::TracingLogger::default())
            // Configure routes
            .service(
                web::scope("/api/v1")
                    .service(handlers::health_check)
                    .service(handlers::generate_mnemonic)
                    .service(handlers::validate_mnemonic)
                    .service(handlers::get_supported_languages)
                    .service(handlers::generate_wallet)
                    .service(handlers::batch_generate_wallets)
                    .service(handlers::get_supported_wallet_types)
                    .service(handlers::test_all_solana_methods)
                    .service(handlers::test_solana_bip32_ed25519)
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}