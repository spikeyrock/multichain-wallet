pub mod auth;
pub mod health;
pub mod mnemonic;
pub mod wallet;

pub use health::HealthServiceImpl;
pub use mnemonic::MnemonicServiceImpl;
pub use wallet::WalletServiceImpl;

// Include the generated protobuf code
pub mod wallet_proto {
    tonic::include_proto!("wallet.v1");
}