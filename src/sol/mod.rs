use rmcp::{
    Error as McpError, RoleServer, ServerHandler, const_string, model::*, schemars,
    service::RequestContext, tool,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

mod check_price;
mod get_balance;
mod model;

#[derive(Debug, Clone)]
pub struct SolanaChad;

#[derive(Debug, Clone, Deserialize, schemars::JsonSchema)]
pub struct WalletBalanceRequest {
    pub address: String,
}

#[derive(Debug, Clone, Serialize, schemars::JsonSchema)]
pub struct SolanaCurrentPrice {
    pub price: f64,
}

#[derive(Debug, Clone, Serialize, schemars::JsonSchema)]
pub struct WalletBalance {
    pub balance: f64,
}

#[tool(tool_box)]
impl SolanaChad {
    pub fn new() -> Self {
        Self
    }

    fn _create_resource_text(&self, uri: &str, name: &str) -> Resource {
        RawResource::new(uri, name.to_string()).no_annotation()
    }

    #[tool(description = "Check current price (USD)")]
    pub async fn check_price(&self) -> Result<CallToolResult, McpError> {
        let price = match check_price::CheckPriceRequest::default().req().await {
            Ok(r) => r,
            Err(e) => return Err(McpError::internal_error(e.to_string(), None)),
        };

        let result = Content::json(SolanaCurrentPrice {
            price: price.solana.usd,
        });

        match result {
            Ok(r) => Ok(CallToolResult::success(vec![r])),
            Err(e) => Err(McpError::invalid_params(e.to_string(), None)),
        }
    }

    #[tool(description = "Check current wallet balance")]
    pub async fn check_balance(
        &self,
        #[tool(aggr)] WalletBalanceRequest { address }: WalletBalanceRequest,
    ) -> Result<CallToolResult, McpError> {
        let res = match get_balance::GetBalanceRequest::new(address).req().await {
            Ok(r) => r,
            Err(e) => return Err(McpError::internal_error(e.to_string(), None)),
        };

        let balance = res.result.value as f64 / 1_000_000_000.0;

        let result = Content::json(WalletBalance { balance });

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
