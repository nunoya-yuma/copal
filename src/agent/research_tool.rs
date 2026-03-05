use std::sync::Arc;

use rig::completion::ToolDefinition;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::any_agent::AnyAgent;

/// Arguments for the ResearchTool
#[derive(Deserialize)]
pub struct ResearchArgs {
    pub topic: String,
}

/// Output from the ResearchTool
#[derive(Serialize)]
pub struct ResearchOutput {
    pub report: String,
}

/// Error type for ResearchTool
#[derive(Debug, thiserror::Error)]
pub enum ResearchError {
    #[error("Research failed: {0}")]
    PromptError(String),
}

fn research_prompt(topic: &str) -> String {
    format!(
        "You are a research assistant. Conduct thorough research on the '{}'.

        Procedure:
        1. Use the web_search tool to run multiple queries and identify reliable sources.
        2. Use the web_fetch tool to retrieve the top 3-5 pages and carefully read their content.
        3. Create a Markdown report in Japanese using the following structure:
                    ## 概要
                    ## 主要な発見
                    ## 詳細分析
                    ## ソース一覧 (URLと概要)",
        topic
    )
}

/// Tool that delegates deep research to an inner agent with web access.
///
/// When the RouterAgent decides a query needs thorough investigation, it calls
/// this tool. The inner AnyAgent then runs its own multi-turn loop using
/// web_search and web_fetch, returning a single structured report.
pub struct ResearchTool {
    inner: Arc<AnyAgent>,
}

impl ResearchTool {
    pub fn new(inner: Arc<AnyAgent>) -> Self {
        Self { inner }
    }
}

impl rig::tool::Tool for ResearchTool {
    const NAME: &'static str = "research_tool";
    type Error = ResearchError;
    type Args = ResearchArgs;
    type Output = ResearchOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: self.name(),
            description: "Conduct deep research on a topic using multiple web sources. \
                          Searches, reads pages, and returns a structured Markdown report \
                          with a summary, key findings, analysis, and sources."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "topic": {
                        "type": "string",
                        "description": "The topic to research thoroughly"
                    }
                },
                "required": ["topic"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let prompt = research_prompt(&args.topic);
        let report = self
            .inner
            .prompt(&prompt)
            .await
            .map_err(|e| ResearchError::PromptError(e.to_string()))?;
        Ok(ResearchOutput { report })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_research_prompt_contains_topic() {
        let result = research_prompt("test-topic");

        assert!(result.contains("test-topic"));
    }

    #[test]
    fn test_research_prompt_mentions_report_structure() {
        let result = research_prompt("test-topic");

        assert!(result.contains("## 概要"));
        assert!(result.contains("## 主要な発見"));
        assert!(result.contains("## 詳細分析"));
        assert!(result.contains("## ソース一覧 (URLと概要)"));
    }
}
