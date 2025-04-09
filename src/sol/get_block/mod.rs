use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::model::{JSON_RPC_VERSION, REQUEST_ID, SOLANA_API_URL, SolanaAPIMethod};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockRequest {
    pub jsonrpc: String,
    pub id: u32,
    pub method: SolanaAPIMethod,
    pub params: (u64, GetBlockParams),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockParams {
    pub encoding: String,
    pub max_supported_transaction_version: u8,
    pub transaction_details: String,
    pub rewards: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockResponse {
    pub jsonrpc: String,
    pub result: Option<BlockResult>,
    pub id: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockResult {
    pub block_height: Option<u64>,
    pub block_time: Option<i64>,
    pub blockhash: String,
    pub parent_slot: u64,
    pub previous_blockhash: String,
    pub transactions: Vec<TransactionWithMeta>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionWithMeta {
    pub meta: serde_json::Value,
    pub transaction: serde_json::Value,
}

impl GetBlockRequest {
    pub fn new(slot: u64) -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id: REQUEST_ID,
            method: SolanaAPIMethod::GetBlock,
            params: (
                slot,
                GetBlockParams {
                    encoding: "jsonParsed".to_string(),
                    max_supported_transaction_version: 0,
                    transaction_details: "full".to_string(),
                    rewards: false,
                },
            ),
        }
    }

    pub async fn fetch(&self) -> Result<GetBlockResponse> {
        let res = reqwest::Client::new()
            .post(SOLANA_API_URL)
            .json(&self)
            .send()
            .await?
            .json::<GetBlockResponse>()
            .await?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_block() {
        let result = GetBlockRequest::new(430).fetch().await;
        assert!(result.is_ok());
    }
}
