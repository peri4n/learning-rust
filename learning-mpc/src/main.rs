use rmcp::{
    ErrorData as McpError, ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
    transport::stdio,
};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct GreetRequest {
    /// The name to greet
    name: String,
}

#[derive(Clone)]
struct GreetServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl GreetServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Greet someone by prefixing "hello" to their name
    #[tool(
        name = "to greet",
        description = "Prefix 'hello' to the given name",
        annotations(read_only_hint = true)
    )]
    async fn to_greet(
        &self,
        Parameters(req): Parameters<GreetRequest>,
    ) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(format!(
            "hello {}",
            req.name,
        ))]))
    }
}

#[tool_handler]
impl ServerHandler for GreetServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::from_build_env())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting MCP greet server");

    let service = GreetServer::new()
        .serve(stdio())
        .await
        .inspect_err(|e| tracing::error!("serving error: {:?}", e))?;

    service.waiting().await?;
    Ok(())
}
