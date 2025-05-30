use serde::{Deserialize, Serialize};
use crate::core::chain_info::{ChainInfo, ChainType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAddress {
    pub address: String,
    pub chain_type: ChainType,
    pub chain_info: ChainInfo,
    pub derivation_path: String,
    pub index: u32,
    pub public_key: String,
    pub private_key: String,
}

#[derive(Debug, Clone)]
pub struct DerivationPath {
    pub purpose: u32,
    pub coin_type: u32,
    pub account: u32,
    pub change: u32,
    pub index: u32,
}

impl DerivationPath {
    pub fn new(purpose: u32, coin_type: u32, account: u32, change: u32, index: u32) -> Self {
        Self {
            purpose,
            coin_type,
            account,
            change,
            index,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "m/{}'/{}'/{}'/{}/{}",
            self.purpose,
            self.coin_type,
            self.account,
            self.change,
            self.index
        )
    }

    // For chains that use hardened derivation on all levels
    pub fn to_string_all_hardened(&self) -> String {
        format!(
            "m/{}'/{}'/{}'/{}'/{}'",
            self.purpose,
            self.coin_type,
            self.account,
            self.change,
            self.index
        )
    }

    // For chains with custom paths
    pub fn to_string_custom(&self, format: &str) -> String {
        format.replace("{purpose}", &self.purpose.to_string())
            .replace("{coin_type}", &self.coin_type.to_string())
            .replace("{account}", &self.account.to_string())
            .replace("{change}", &self.change.to_string())
            .replace("{index}", &self.index.to_string())
    }
}