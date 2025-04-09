use anyhow::Result;
use serde::{Deserialize, Serialize};

pub struct GetPriceRequest;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetPriceResponse {
    pub solana: SolanaPriceInfo,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SolanaPriceInfo {
    pub usd: f64,
}

impl GetPriceRequest {
    pub fn new() -> Self {
        Self
    }

    pub async fn fetch(&self) -> Result<GetPriceResponse> {
        let res = reqwest::Client::new()
            .get("https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd")
            .send()
            .await?
            .json::<GetPriceResponse>()
            .await?;

        Ok(res)
    }
}

impl Default for GetPriceRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_price() {
        let result = GetPriceRequest::default().fetch().await;
        assert!(result.is_ok());
        assert!(result.unwrap().solana.usd > 0.0);
    }
}
