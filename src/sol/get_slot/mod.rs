use serde::{Deserialize, Serialize};

use super::model::{JSON_RPC_VERSION, REQUEST_ID, SOLANA_API_URL, SolanaAPIMethod};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSlotRequest {
    pub jsonrpc: String,
    pub id: u32,
    pub method: SolanaAPIMethod,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSlotResponse {
    pub jsonrpc: String,
    pub result: u64, // slot number
    pub id: u32,
}

impl GetSlotRequest {
    pub fn new() -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id: REQUEST_ID,
            method: SolanaAPIMethod::GetSlot,
        }
    }

    pub async fn fetch(&self) -> Result<GetSlotResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let response = client.post(SOLANA_API_URL).json(self).send().await?;

        let result = response.json::<GetSlotResponse>().await?;
        Ok(result)
    }
}

impl Default for GetSlotRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_slot() {
        let result = GetSlotRequest::default().fetch().await;
        assert!(result.is_ok());
        assert!(result.unwrap().result > 0);
    }
}
