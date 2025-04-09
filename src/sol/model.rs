use serde::{Deserialize, Serialize};

pub const SOLANA_API_URL: &str = "https://api.mainnet-beta.solana.com";
pub const JSON_RPC_VERSION: &str = "2.0";
pub const REQUEST_ID: u32 = 1;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Context {
    slot: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SolanaAPIMethod {
    GetHealth,
    GetBalance,
    GetBlock,
    GetBlockHeight,
    GetSlot,
    GetSupply,
}

impl std::fmt::Display for SolanaAPIMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolanaAPIMethod::GetHealth => write!(f, "getHealth"),
            SolanaAPIMethod::GetBalance => write!(f, "getBalance"),
            SolanaAPIMethod::GetBlock => write!(f, "getBlock"),
            SolanaAPIMethod::GetBlockHeight => write!(f, "getBlockHeight"),
            SolanaAPIMethod::GetSlot => write!(f, "getSlot"),
            SolanaAPIMethod::GetSupply => write!(f, "getSupply"),
        }
    }
}
