use rmcp::{
    Error as McpError, RoleServer, ServerHandler, const_string, model::*, schemars,
    service::RequestContext, tool,
};
use serde::Deserialize;
use serde_json::json;
use ta::Next;
use ta::indicators::{MovingAverageConvergenceDivergence as Macd, RelativeStrengthIndex as Rsi};

mod get_balance;
mod get_block;
mod get_block_height;
mod get_health;
mod get_market_chart;
mod get_price;
mod get_slot;
mod get_supply;
mod model;

#[derive(Debug, Clone)]
pub struct SolanaChad;

#[derive(Debug, Clone, Deserialize, schemars::JsonSchema)]
pub struct WalletBalanceRequest {
    pub address: String,
}

#[derive(Debug, Clone, Deserialize, schemars::JsonSchema)]
pub struct GetBlockRequest {
    pub slot: u64,
}

#[derive(Debug, Clone, Deserialize, schemars::JsonSchema)]
pub struct GetMarketChartRequest {
    pub days: u32,
}

#[tool(tool_box)]
impl SolanaChad {
    pub fn new() -> Self {
        Self
    }

    fn _create_resource_text(&self, uri: &str, name: &str) -> Resource {
        RawResource::new(uri, name.to_string()).no_annotation()
    }

    #[tool(description = "Get health status")]
    pub async fn get_health(&self) -> Result<CallToolResult, McpError> {
        let res = match get_health::GetHealthRequest::default().fetch().await {
            Ok(r) => r,
            Err(e) => return Err(McpError::internal_error(e.to_string(), None)),
        };

        let result = Content::json(res);

        match result {
            Ok(r) => Ok(CallToolResult::success(vec![r])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Get current price (USD)")]
    pub async fn get_price(&self) -> Result<CallToolResult, McpError> {
        let price = match get_price::GetPriceRequest::default().fetch().await {
            Ok(r) => r,
            Err(e) => return Err(McpError::internal_error(e.to_string(), None)),
        };

        let result = Content::json(price);

        match result {
            Ok(r) => Ok(CallToolResult::success(vec![r])),
            Err(e) => Err(McpError::invalid_params(e.to_string(), None)),
        }
    }

    #[tool(description = "Get current wallet balance")]
    pub async fn get_balance(
        &self,
        #[tool(aggr)] WalletBalanceRequest { address }: WalletBalanceRequest,
    ) -> Result<CallToolResult, McpError> {
        let res = match get_balance::GetBalanceRequest::new(address).fetch().await {
            Ok(r) => r,
            Err(e) => return Err(McpError::internal_error(e.to_string(), None)),
        };

        let result = Content::json(res);

        match result {
            Ok(r) => Ok(CallToolResult::success(vec![r])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Get block by slot")]
    pub async fn get_block(
        &self,
        #[tool(aggr)] GetBlockRequest { slot }: GetBlockRequest,
    ) -> Result<CallToolResult, McpError> {
        let res = match get_block::GetBlockRequest::new(slot).fetch().await {
            Ok(r) => r,
            Err(e) => return Err(McpError::internal_error(e.to_string(), None)),
        };

        let result = Content::json(res);

        match result {
            Ok(r) => Ok(CallToolResult::success(vec![r])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Get block height")]
    pub async fn get_block_height(&self) -> Result<CallToolResult, McpError> {
        let res = match get_block_height::GetBlockHeightRequest::default()
            .fetch()
            .await
        {
            Ok(r) => r,
            Err(e) => return Err(McpError::internal_error(e.to_string(), None)),
        };

        let result = Content::json(res);

        match result {
            Ok(r) => Ok(CallToolResult::success(vec![r])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Get current slot")]
    pub async fn get_slot(&self) -> Result<CallToolResult, McpError> {
        let res = match get_slot::GetSlotRequest::default().fetch().await {
            Ok(r) => r,
            Err(e) => return Err(McpError::internal_error(e.to_string(), None)),
        };

        let result = Content::json(res);

        match result {
            Ok(r) => Ok(CallToolResult::success(vec![r])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Get current supply")]
    pub async fn get_supply(&self) -> Result<CallToolResult, McpError> {
        let res = match get_supply::GetSupplyRequest::default().fetch().await {
            Ok(r) => r,
            Err(e) => return Err(McpError::internal_error(e.to_string(), None)),
        };

        let result = Content::json(res);

        match result {
            Ok(r) => Ok(CallToolResult::success(vec![r])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Get MACD chart")]
    pub async fn get_macd_chart(
        &self,
        #[tool(aggr)] GetMarketChartRequest { days }: GetMarketChartRequest,
    ) -> Result<CallToolResult, McpError> {
        let res = match get_market_chart::MarketChartRequest::new(days)
            .fetch()
            .await
        {
            Ok(r) => r,
            Err(e) => return Err(McpError::internal_error(e.to_string(), None)),
        };

        // Deserialize market chart
        let prices = res
            .prices
            .iter()
            .map(|entry| entry[1]) // [timestamp, price] → เอา price
            .collect::<Vec<f64>>();

        let mut macd = Macd::new(12, 26, 9).unwrap(); // default MACD config

        // Run MACD
        let mut macd_points = Vec::new();

        for price in prices {
            let macd_val = macd.next(price);

            if macd_val.macd.is_finite()
                && macd_val.signal.is_finite()
                && macd_val.histogram.is_finite()
                && price.is_finite()
            {
                macd_points.push(json!({
                    "price": price,
                    "macd": macd_val.macd,
                    "signal": macd_val.signal,
                    "histogram": macd_val.histogram,
                }));
            } else {
                tracing::info!(
                    "⚠️ MACD infinite/NaN found: price={}, macd={:?}, signal={:?}, histogram={:?}",
                    price,
                    macd_val.macd,
                    macd_val.signal,
                    macd_val.histogram
                );
            }
        }

        let result = Content::json(json!({
            "macd_chart": macd_points
        }));

        match result {
            Ok(r) => Ok(CallToolResult::success(vec![r])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Get RSI chart")]
    pub async fn get_rsi_chart(
        &self,
        #[tool(aggr)] GetMarketChartRequest { days }: GetMarketChartRequest,
    ) -> Result<CallToolResult, McpError> {
        let res = match get_market_chart::MarketChartRequest::new(days)
            .fetch()
            .await
        {
            Ok(r) => r,
            Err(e) => return Err(McpError::internal_error(e.to_string(), None)),
        };

        // Extract only finite close prices
        let prices = res
            .prices
            .iter()
            .filter_map(|entry| {
                let price = entry[1];
                if price.is_finite() && price > 0.0 {
                    Some(price)
                } else {
                    tracing::info!("⚠️ Invalid price skipped: {}", price);
                    None
                }
            })
            .collect::<Vec<f64>>();

        let mut rsi = Rsi::new(14).unwrap(); // RSI 14 periods
        let mut rsi_points = Vec::new();
        for price in prices {
            let rsi_val = rsi.next(price);

            if rsi_val.is_finite() {
                rsi_points.push(json!({
                    "price": price,
                    "rsi": rsi_val
                }));
            } else {
                println!("⚠️ Non-finite RSI detected: {}", rsi_val);
            }
        }

        let result = Content::json(json!({
            "rsi_chart": rsi_points
        }));

        match result {
            Ok(r) => Ok(CallToolResult::success(vec![r])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }
}

const_string!(Echo = "echo");
#[tool(tool_box)]
impl ServerHandler for SolanaChad {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "This server is provide real-time money hack within 'set_money'".to_string(),
            ),
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![
                self._create_resource_text("str:////Users/to/some/path/", "cwd"),
                self._create_resource_text("memo://insights", "memo-name"),
            ],
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        match uri.as_str() {
            "str:////Users/to/some/path/" => {
                let cwd = "/Users/to/some/path/";
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(cwd, uri)],
                })
            }
            "memo://insights" => {
                let memo = "Business Intelligence Memo\n\nAnalysis has revealed 5 key insights ...";
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(memo, uri)],
                })
            }
            _ => Err(McpError::resource_not_found(
                "resource_not_found",
                Some(json!({
                    "uri": uri
                })),
            )),
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        Ok(ListPromptsResult {
            next_cursor: None,
            prompts: vec![Prompt::new(
                "example_prompt",
                Some("This is an example prompt that takes one required argument, message"),
                Some(vec![PromptArgument {
                    name: "message".to_string(),
                    description: Some("A message to put in the prompt".to_string()),
                    required: Some(true),
                }]),
            )],
        })
    }

    async fn get_prompt(
        &self,
        GetPromptRequestParam { name, arguments }: GetPromptRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        match name.as_str() {
            "example_prompt" => {
                let message = arguments
                    .and_then(|json| json.get("message")?.as_str().map(|s| s.to_string()))
                    .ok_or_else(|| {
                        McpError::invalid_params("No message provided to example_prompt", None)
                    })?;

                let prompt =
                    format!("This is an example prompt with your message here: '{message}'");
                Ok(GetPromptResult {
                    description: None,
                    messages: vec![PromptMessage {
                        role: PromptMessageRole::User,
                        content: PromptMessageContent::text(prompt),
                    }],
                })
            }
            _ => Err(McpError::invalid_params("prompt not found", None)),
        }
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            next_cursor: None,
            resource_templates: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_macd_chart_success() {
        let solana_chad = SolanaChad::new();
        let request = GetMarketChartRequest { days: 7 };
        let result = solana_chad.get_macd_chart(request).await;
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);

        let result = result.unwrap();
        tracing::info!("Result: {:?}", result);
    }

    #[tokio::test]
    async fn test_get_rsi_chart_success() {
        let solana_chad = SolanaChad::new();
        let request = GetMarketChartRequest { days: 7 };
        let result = solana_chad.get_rsi_chart(request).await;
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);

        let result = result.unwrap();
        tracing::info!("Result: {:?}", result);
    }
}
