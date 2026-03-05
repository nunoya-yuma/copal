pub mod any_agent;
mod builder;
mod chat_agent;
mod pdf_read;
pub(crate) mod research_tool;
pub mod router_agent;
mod web_fetch;
mod web_search;

#[cfg(test)]
pub mod mock_agent;

pub use any_agent::{AnyAgent, ChatStreamEvent};
pub use builder::{
    create_gemini_agent, create_gemini_router_agent, create_ollama_agent,
    create_ollama_router_agent, create_openai_agent, create_openai_router_agent, default_model,
};
pub use chat_agent::ChatAgent;
pub use pdf_read::PdfRead;
pub use router_agent::RouterAgent;
pub use web_fetch::WebFetch;
pub use web_search::{WebSearch, WebSearchArgs};

#[cfg(test)]
pub use mock_agent::MockAgent;
