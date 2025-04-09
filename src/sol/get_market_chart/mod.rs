use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketChartRequest {
    pub days: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketChartResponse {
    pub prices: Vec<[f64; 2]>,        // [timestamp, price]
    pub market_caps: Vec<[f64; 2]>,   // [timestamp, market cap]
    pub total_volumes: Vec<[f64; 2]>, // [timestamp, total volume]
}

impl MarketChartRequest {
    pub fn new(days: u32) -> Self {
        Self { days }
    }

    pub async fn fetch(&self) -> Result<MarketChartResponse, reqwest::Error> {
        let url = format!(
            "https://api.coingecko.com/api/v3/coins/solana/market_chart?vs_currency=usd&days={}&interval=daily",
            self.days
        );
        let response = reqwest::Client::new().get(&url).send().await?;
        let result = response.json::<MarketChartResponse>().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_market_chart() {
        let result = MarketChartRequest::new(0).fetch().await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(!data.prices.is_empty());
        assert!(!data.market_caps.is_empty());
        assert!(!data.total_volumes.is_empty());
    }
}
