use serde::{Deserialize, Serialize};

use super::model::{JSON_RPC_VERSION, REQUEST_ID, SOLANA_API_URL, SolanaAPIMethod};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockHeightRequest {
    pub jsonrpc: String,
    pub id: u32,
    pub method: SolanaAPIMethod,
    pub params: Vec<()>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockHeightResponse {
    pub jsonrpc: String,
    pub result: u64, // block height
    pub id: u32,
}

impl GetBlockHeightRequest {
    pub fn new() -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id: REQUEST_ID,
            method: SolanaAPIMethod::GetBlockHeight,
            params: vec![],
        }
    }

    pub async fn fetch(&self) -> Result<GetBlockHeightResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .post(SOLANA_API_URL)
            .json(self)
            .send()
            .await?
            .json::<GetBlockHeightResponse>()
            .await?;

        Ok(res)
    }
}

impl Default for GetBlockHeightRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_block_height() {
        let result = GetBlockHeightRequest::default().fetch().await;
        assert!(result.is_ok());
        assert!(result.unwrap().result > 0);
    }
}
