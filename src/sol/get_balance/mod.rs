use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::model::{Context, JSON_RPC_VERSION, REQUEST_ID, SOLANA_API_URL, SolanaAPIMethod};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {
    jsonrpc: String,
    id: u32,
    method: SolanaAPIMethod,
    params: Vec<String>, // Address
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    pub jsonrpc: String,
    pub result: GetBalanceResponseResult,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponseResult {
    pub context: Context,
    pub value: i64,
}

impl GetBalanceRequest {
    pub fn new(address: String) -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id: REQUEST_ID,
            method: SolanaAPIMethod::GetBalance,
            params: vec![address],
        }
    }

    pub async fn fetch(&self) -> Result<GetBalanceResponse> {
        let res = reqwest::Client::new()
            .post(SOLANA_API_URL)
            .json(&self)
            .send()
            .await?
            .json::<GetBalanceResponse>()
            .await?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_balance() {
        let address = "83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri".to_string();
        let result = GetBalanceRequest::new(address).fetch().await;
        assert!(result.is_ok());
        assert!(result.unwrap().result.value >= 0);
    }
}
