use serde::{Deserialize, Serialize};

use super::model::{JSON_RPC_VERSION, REQUEST_ID, SOLANA_API_URL, SolanaAPIMethod};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSupplyRequest {
    pub jsonrpc: String,
    pub id: u32,
    pub method: SolanaAPIMethod,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSupplyResponse {
    pub jsonrpc: String,
    pub result: SupplyResult,
    pub id: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplyResult {
    pub context: Context,
    pub value: SupplyValue,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub slot: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplyValue {
    pub total: u64,
    pub circulating: u64,
    pub non_circulating: u64,
    pub non_circulating_accounts: Vec<String>,
}

impl GetSupplyRequest {
    pub fn new() -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id: REQUEST_ID,
            method: SolanaAPIMethod::GetSupply,
        }
    }

    pub async fn fetch(&self) -> Result<GetSupplyResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let response = client.post(SOLANA_API_URL).json(self).send().await?;

        let result = response.json::<GetSupplyResponse>().await?;
        Ok(result)
    }
}

impl Default for GetSupplyRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_supply() {
        let result = GetSupplyRequest::default().fetch().await;
        assert!(result.is_ok());
        let supply_result = result.unwrap();
        assert!(supply_result.result.value.total > 0);
    }
}
