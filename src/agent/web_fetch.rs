use log::info;
use rig::completion::ToolDefinition;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::collectors::robots::RobotsCache;
use crate::collectors::web::fetch_url;

/// Arguments for the WebFetch tool
#[derive(Deserialize)]
pub struct WebFetchArgs {
    url: String,
}

/// Output from the WebFetch tool
#[derive(Serialize)]
pub struct WebFetchOutput {
    title: Option<String>,
    content: String,
}

/// Error type for WebFetch tool
#[derive(Debug, thiserror::Error)]
pub enum WebFetchError {
    #[error("Failed to fetch URL: {0}")]
    FetchError(#[from] anyhow::Error),
}

/// Web page fetcher with shared robots.txt cache.
/// Clone shares the same cache via Arc, enabling cache reuse across agents.
#[derive(Clone)]
pub struct WebFetch {
    robots_cache: RobotsCache,
}

impl Default for WebFetch {
    fn default() -> Self {
        Self::new()
    }
}

impl WebFetch {
    pub fn new() -> Self {
        Self {
            robots_cache: RobotsCache::new(),
        }
    }
}

impl rig::tool::Tool for WebFetch {
    const NAME: &'static str = "web_fetch";
    type Error = WebFetchError;
    type Args = WebFetchArgs;
    type Output = WebFetchOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: self.name(),
            description: "Fetches content from a web URL".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The URL to fetch"
                    }
                },
                "required": ["url"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        info!("Fetching {} ...", args.url);
        let page = fetch_url(&args.url, &self.robots_cache).await?;
        Ok(WebFetchOutput {
            title: page.title,
            content: page.text,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rig::tool::Tool;

    #[test]
    fn test_web_fetch_args_deserialize() {
        let json = r#"{"url": "https://example.com"}"#;
        let args: WebFetchArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.url, "https://example.com");
    }

    #[test]
    fn test_web_fetch_output_serialize() {
        let output = WebFetchOutput {
            title: Some("Test".to_string()),
            content: "Hello".to_string(),
        };
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("Test"));
    }

    #[tokio::test]
    #[ignore]
    async fn test_web_fetch_tool_get_example_url() {
        let sut = WebFetch::new();
        let json = r#"{"url": "https://example.com"}"#;
        let args: WebFetchArgs = serde_json::from_str(json).unwrap();

        let output = sut.call(args).await.unwrap();

        assert_eq!(output.title, Some("Example Domain".to_string()));
        assert!(!output.content.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn test_web_fetch_tool_fail_to_get_web_page() {
        let sut = WebFetch::new();
        let json = r#"{"url": "https://lobalhost"}"#;
        let args: WebFetchArgs = serde_json::from_str(json).unwrap();

        let output = sut.call(args).await;
        assert!(output.is_err());
    }
}
