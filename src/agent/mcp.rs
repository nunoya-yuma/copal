use anyhow::Result;
use log::{info, warn};
use rmcp::transport::TokioChildProcess;
use rmcp::ServiceExt;
use serde::Deserialize;

/// Configuration for a single MCP server.
///
/// The `type` field selects the transport:
/// - `"stdio"`: spawn a local subprocess (most common, e.g. npx servers)
/// - `"http"`:  connect to a remote server by URL
///
/// # Example env var (`MCP_SERVERS`)
/// ```json
/// [
///   {"type": "stdio", "command": "npx", "args": ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]},
///   {"type": "http",  "url": "https://my-mcp-server.example.com/mcp"}
/// ]
/// ```
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum McpServerConfig {
    Stdio { command: String, args: Vec<String> },
    Http { url: String },
}

/// A connected MCP tool set: tool definitions + the sink to invoke them.
///
/// Both fields implement `Clone`, so one `McpToolSet` can be cheaply shared
/// between the outer `RouterAgent` and the inner `AnyAgent`.
#[derive(Clone)]
pub struct McpToolSet {
    pub tools: Vec<rmcp::model::Tool>,
    pub sink: rmcp::service::ServerSink,
}

/// Parse the `MCP_SERVERS` environment variable and connect to each server.
///
/// Returns one `McpToolSet` per successfully connected server.
/// Servers that fail to start are logged as warnings and skipped.
pub async fn load_mcp_tools() -> Vec<McpToolSet> {
    let mcp_json = match std::env::var("MCP_SERVERS") {
        Ok(s) => s,
        Err(_) => return vec![],
    };

    let configs: Vec<McpServerConfig> = match serde_json::from_str(&mcp_json) {
        Ok(c) => c,
        Err(e) => {
            warn!("Failed to deserialize MCP_SERVERS: {e}");
            return vec![];
        }
    };

    let mut mcp_tool_set = vec![];
    for config in configs {
        match connect_mcp_server(config.clone()).await {
            Ok(m) => {
                info!(
                    "Connected to MCP server ({config:?}): {} tool(s) available",
                    m.tools.len()
                );
                mcp_tool_set.push(m);
            }
            Err(e) => warn!("Failed to connect to MCP server ({config:?}): {e}"),
        }
    }
    mcp_tool_set
}

/// Connect to a single MCP server and return its tools + sink.
async fn connect_mcp_server(config: McpServerConfig) -> Result<McpToolSet> {
    let client = match config {
        McpServerConfig::Stdio { command, args } => {
            let mut cmd = tokio::process::Command::new(command);
            cmd.args(args);
            let transport = TokioChildProcess::new(cmd)?;
            ().serve(transport).await?
        }
        McpServerConfig::Http { url } => {
            let transport = rmcp::transport::StreamableHttpClientTransport::from_uri(url);
            ().serve(transport).await?
        }
    };

    let tools = client.list_tools(Default::default()).await?.tools;
    let sink = client.peer().to_owned();
    Ok(McpToolSet { tools, sink })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdio_config_deserializes() {
        let json = r#"{"type":"stdio","command":"npx","args":["-y","@modelcontextprotocol/server-filesystem","/tmp"]}"#;

        let config: McpServerConfig = serde_json::from_str(json).unwrap();

        let McpServerConfig::Stdio { command, args } = config else {
            panic!("Expected Stdio variant");
        };
        assert_eq!(command, "npx");
        assert_eq!(
            args,
            ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]
        );
    }

    #[test]
    fn test_http_config_deserializes() {
        let json = r#"{"type":"http","url":"https://example.com/mcp"}"#;
        let config: McpServerConfig = serde_json::from_str(json).unwrap();

        let McpServerConfig::Http { url } = config else {
            panic!("Expected Http variant");
        };
        assert_eq!(url, "https://example.com/mcp");
    }
    #[test]
    fn test_multiple_server_configs_deserialize() {
        let json = r#"[
          {"type": "stdio", "command": "npx", "args": ["-y", "server-a"]},
          {"type": "http",  "url": "https://example.com/mcp"}
        ]"#;
        let configs: Vec<McpServerConfig> = serde_json::from_str(json).unwrap();
        assert_eq!(configs.len(), 2);
    }

    #[tokio::test]
    async fn returns_empty_when_mcp_servers_is_empty_string() {
        std::env::set_var("MCP_SERVERS", "");

        let result = load_mcp_tools().await;

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn returns_empty_when_mcp_servers_is_empty_array() {
        std::env::set_var("MCP_SERVERS", "[]");

        let result = load_mcp_tools().await;

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn returns_empty_when_no_mcp_servers_configured() {
        std::env::remove_var("MCP_SERVERS");

        let mcp_list = load_mcp_tools().await;

        assert!(mcp_list.is_empty());
    }

    #[tokio::test]
    async fn returns_empty_when_mcp_servers_is_invalid_json() {
        std::env::set_var("MCP_SERVERS", "not-json");

        let mcp_list = load_mcp_tools().await;

        assert!(mcp_list.is_empty());
    }

    #[test]
    fn fails_to_deserialize_unknown_transport_type() {
        let json = r#"{"type":"ftp","url":"ftp://example.com"}"#;

        let result = serde_json::from_str::<McpServerConfig>(json);

        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore = "Requires npx and @modelcontextprotocol packages installed"]
    async fn test_load_mcp_tools_with_multiple_servers() {
        std::env::set_var(
            "MCP_SERVERS",
            r#"[
                        {"type": "stdio", "command": "npx", "args": ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]},
                        {"type": "stdio", "command": "npx", "args": ["-y", "@modelcontextprotocol/server-time"]}
                    ]"#,
        );
        let mcp_list = load_mcp_tools().await;
        assert_eq!(mcp_list.len(), 2);
    }

    #[tokio::test]
    #[ignore = "Requires a real HTTP MCP server. Update `url` below before running."]
    async fn test_connect_mcp_server_http() {
        // Requires a real HTTP MCP server.
        let url = "http://localhost:8080/mcp".to_string(); // Replace with actual server URL
        let config = McpServerConfig::Http { url };
        let result = connect_mcp_server(config).await;
        assert!(result.is_ok());
    }
}
