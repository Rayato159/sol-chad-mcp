use serde::{Deserialize, Serialize};

pub const SOLANA_PRICE_CHECK_URL: &str =
    "https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd";
pub const SOLANA_API_URL: &str = "https://api.mainnet-beta.solana.com";
pub const JSON_RPC_VERSION: &str = "2.0";
pub const REQUEST_ID: i64 = 1;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Context {
    slot: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SolanaAPIMethod {
    GetBalance,
}

impl std::fmt::Display for SolanaAPIMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolanaAPIMethod::GetBalance => write!(f, "getBalance"),
        }
    }
}
