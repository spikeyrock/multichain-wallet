use std::sync::Arc;

use crate::core::{Chain, ChainType};

// Chain modules
pub mod bitcoin;
pub mod ethereum;
pub mod ripple;
pub mod solana;
pub mod tron;
pub mod sui;
pub mod stellar;
pub mod near;
pub mod ton;
pub mod dogecoin;
pub mod polkadot;
pub mod cosmos;
pub mod tezos;
pub mod eos;
pub mod hedera;
pub mod filecoin;
pub mod mina;
pub mod icp;

// Re-export for convenience
pub use bitcoin::{BitcoinLegacy, BitcoinSegwit, BitcoinTaproot};
pub use ethereum::Ethereum;
pub use ripple::Ripple;
pub use solana::Solana;
pub use tron::Tron;
pub use sui::Sui;
pub use stellar::Stellar;
pub use near::Near;
pub use ton::Ton;
pub use dogecoin::Dogecoin;
pub use polkadot::Polkadot;
pub use cosmos::CosmosChain;
pub use tezos::Tezos;
pub use eos::Eos;
pub use hedera::Hedera;
pub use filecoin::Filecoin;
pub use mina::Mina;
pub use icp::InternetComputer;

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
        ChainType::Sui => Arc::new(Sui::new()),
        ChainType::Stellar => Arc::new(Stellar::new()),
        ChainType::Near => Arc::new(Near::new()),
        ChainType::Ton => Arc::new(Ton::new()),
        ChainType::Dogecoin => Arc::new(Dogecoin::new(Network::Bitcoin)),
        ChainType::Polkadot => Arc::new(Polkadot::new()),
        ChainType::Cosmos => Arc::new(CosmosChain::new(ChainType::Cosmos)),
        ChainType::Osmosis => Arc::new(CosmosChain::new(ChainType::Osmosis)),
        ChainType::Juno => Arc::new(CosmosChain::new(ChainType::Juno)),
        ChainType::Secret => Arc::new(CosmosChain::new(ChainType::Secret)),
        ChainType::Akash => Arc::new(CosmosChain::new(ChainType::Akash)),
        ChainType::Sei => Arc::new(CosmosChain::new(ChainType::Sei)),
        ChainType::Celestia => Arc::new(CosmosChain::new(ChainType::Celestia)),
        ChainType::Injective => Arc::new(CosmosChain::new(ChainType::Injective)),
        ChainType::Tezos => Arc::new(Tezos::new()),
        ChainType::Eos => Arc::new(Eos::new()),
        ChainType::Hedera => Arc::new(Hedera::new()),
        ChainType::Filecoin => Arc::new(Filecoin::new()),
        ChainType::Mina => Arc::new(Mina::new()),
        ChainType::InternetComputer => Arc::new(InternetComputer::new()),
    }
}