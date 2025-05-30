use async_trait::async_trait;
use crate::core::types::{WalletAddress, DerivationPath};
use crate::core::chain_info::ChainInfo;
use crate::errors::ApiResult;

#[async_trait]
pub trait Chain: Send + Sync {
    /// Get chain information
    fn info(&self) -> ChainInfo;
    
    /// Generate a wallet address from seed
    async fn generate_address(
        &self,
        seed: &[u8],
        passphrase: &str,
        index: u32,
    ) -> ApiResult<WalletAddress>;
    
    /// Get the derivation path for this chain
    fn derivation_path(&self, index: u32) -> DerivationPath;
    
    /// Validate an address for this chain
    async fn validate_address(&self, address: &str) -> bool;
    
    /// Get example address for documentation
    fn example_address(&self) -> &str {
        "example_address"
    }
}