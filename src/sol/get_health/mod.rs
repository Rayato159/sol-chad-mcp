use serde::{Deserialize, Serialize};

use super::model::{JSON_RPC_VERSION, REQUEST_ID, SOLANA_API_URL, SolanaAPIMethod};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHealthRequest {
    pub jsonrpc: String,
    pub id: u32,
    pub method: SolanaAPIMethod,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum GetHealthResponse {
    Ok {
        jsonrpc: String,
        result: String, // "ok"
        id: u32,
    },
    Err {
        jsonrpc: String,
        error: RpcError,
        id: u32,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}

impl GetHealthRequest {
    pub fn new() -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id: REQUEST_ID,
            method: SolanaAPIMethod::GetHealth,
        }
    }

    pub async fn fetch(&self) -> Result<GetHealthResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let response = client.post(SOLANA_API_URL).json(self).send().await?;

        let result = response.json::<GetHealthResponse>().await?;
        Ok(result)
    }
}

impl Default for GetHealthRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_health() {
        let result = GetHealthRequest::default().fetch().await.unwrap();
        match result {
            GetHealthResponse::Ok { result, .. } => {
                assert_eq!(result, "ok");
            }
            GetHealthResponse::Err { error, .. } => {
                panic!("Error: {:?}", error);
            }
        }
    }
}
