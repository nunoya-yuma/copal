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

// TODO(human): Implement research_prompt(topic: &str) -> String
//
// This function builds the instruction given to the inner research sub-agent.
// The inner agent already has web_search and web_fetch tools available and will
// run multiple tool-calling turns before returning.
//
// Design this prompt to guide the sub-agent toward:
//   - Running multiple web_search queries with varied keywords
//   - Fetching and reading the top 3-5 pages with web_fetch
//   - Returning a structured Markdown report with the following sections:
//       ## 概要
//       ## 主要な発見
//       ## 詳細分析
//       ## ソース一覧（URLと概要）
//
// Hint: the existing research_prompt in src/web/handlers.rs (now simplified) is
// a good reference — but tailor this one specifically for the sub-agent context
// where `topic` is an explicit argument.
//
// After implementing, write these two tests in the #[cfg(test)] block below:
//   test_research_prompt_contains_topic
//   test_research_prompt_mentions_report_structure
fn research_prompt(_topic: &str) -> String {
    todo!("TODO(human): implement research_prompt")
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

    // TODO(human): Write two tests for research_prompt:
    //
    // Test 1: test_research_prompt_contains_topic
    //   - Call research_prompt("量子コンピュータ")
    //   - Assert the returned string contains "量子コンピュータ"
    //
    // Test 2: test_research_prompt_mentions_report_structure
    //   - Call research_prompt("任意のトピック")
    //   - Assert the returned string contains "## 概要" and "## ソース一覧"
}
