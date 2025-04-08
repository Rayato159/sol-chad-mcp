use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::model::SOLANA_PRICE_CHECK_URL;

pub struct CheckPriceRequest;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckPriceResponse {
    pub solana: SolanaPriceInfo,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SolanaPriceInfo {
    pub usd: f64,
}

impl CheckPriceRequest {
    pub fn new() -> Self {
        Self
    }

    pub async fn req(&self) -> Result<CheckPriceResponse> {
        let res = reqwest::Client::new()
            .get(SOLANA_PRICE_CHECK_URL)
            .send()
            .await?
            .json::<CheckPriceResponse>()
            .await?;

        Ok(res)
    }
}

impl Default for CheckPriceRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_price() {
        let result = CheckPriceRequest::default().req().await;
        assert!(result.is_ok());
        assert!(result.unwrap().solana.usd > 0.0);
    }
}
