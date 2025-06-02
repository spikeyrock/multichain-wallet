use std::sync::Arc;

use crate::core::{Chain, ChainType};

// Chain modules
pub mod bitcoin;
pub mod ethereum;
pub mod ripple;
pub mod solana;
pub mod tron;
pub mod sui;
pub mod near;
pub mod dogecoin;
pub mod cosmos;
pub mod tezos;
pub mod filecoin;

// Re-export for convenience
pub use bitcoin::{BitcoinLegacy, BitcoinSegwit, BitcoinTaproot};
pub use ethereum::Ethereum;
pub use ripple::Ripple;
pub use solana::Solana;
pub use tron::Tron;
pub use sui::Sui;
pub use near::Near;
pub use dogecoin::Dogecoin;
pub use cosmos::CosmosChain;
pub use tezos::Tezos;
pub use filecoin::Filecoin;

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
        ChainType::Near => Arc::new(Near::new()),
        ChainType::Dogecoin => Arc::new(Dogecoin::new(Network::Bitcoin)),
        ChainType::Cosmos => Arc::new(CosmosChain::new(ChainType::Cosmos)),
        ChainType::Osmosis => Arc::new(CosmosChain::new(ChainType::Osmosis)),
        ChainType::Juno => Arc::new(CosmosChain::new(ChainType::Juno)),
        ChainType::Secret => Arc::new(CosmosChain::new(ChainType::Secret)),
        ChainType::Akash => Arc::new(CosmosChain::new(ChainType::Akash)),
        ChainType::Sei => Arc::new(CosmosChain::new(ChainType::Sei)),
        ChainType::Celestia => Arc::new(CosmosChain::new(ChainType::Celestia)),
        ChainType::Injective => Arc::new(CosmosChain::new(ChainType::Injective)),
        ChainType::Tezos => Arc::new(Tezos::new()),
        ChainType::Filecoin => Arc::new(Filecoin::new()),
    }
}