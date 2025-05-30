use std::sync::Arc;

use crate::core::{Chain, ChainType};

// Chain modules
pub mod bitcoin;
pub mod ethereum;
pub mod ripple;
pub mod solana;
pub mod tron;
pub mod cardano;
pub mod sui;
pub mod stellar;
pub mod monero;
pub mod near;
pub mod ton;
pub mod dogecoin;
pub mod polkadot;

// Re-export for convenience
pub use bitcoin::{BitcoinLegacy, BitcoinSegwit, BitcoinTaproot};
pub use ethereum::Ethereum;
pub use ripple::Ripple;
pub use solana::Solana;
pub use tron::Tron;
pub use cardano::Cardano;
pub use sui::Sui;
pub use stellar::Stellar;
pub use monero::Monero;
pub use near::Near;
pub use ton::Ton;
pub use dogecoin::Dogecoin;
pub use polkadot::Polkadot;

/// Create a chain instance based on the chain type
pub fn create_chain(chain_type: &ChainType) -> Arc<dyn Chain> {
    use crate::chains::bitcoin::Network;
    
    match chain_type {
        ChainType::BitcoinLegacy => Arc::new(BitcoinLegacy::new(Network::Bitcoin)),
        ChainType::BitcoinSegwit => Arc::new(BitcoinSegwit::new(Network::Bitcoin)),
        ChainType::BitcoinTaproot => Arc::new(BitcoinTaproot::new(Network::Bitcoin)),
        ChainType::Ethereum => Arc::new(Ethereum::new()),
        ChainType::Ripple => Arc::new(Ripple::new()),
        ChainType::Solana => Arc::new(Solana::new()),
        ChainType::Tron => Arc::new(Tron::new()),
        ChainType::Cardano => Arc::new(Cardano::new()),
        ChainType::Sui => Arc::new(Sui::new()),
        ChainType::Stellar => Arc::new(Stellar::new()),
        ChainType::Monero => Arc::new(Monero::new()),
        ChainType::Near => Arc::new(Near::new()),
        ChainType::Ton => Arc::new(Ton::new()),
        ChainType::Dogecoin => Arc::new(Dogecoin::new(Network::Bitcoin)),
        ChainType::Polkadot => Arc::new(Polkadot::new()),
    }
}