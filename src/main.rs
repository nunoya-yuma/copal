use clap::Parser;
use dotenvy::dotenv;
use log::error;
use rig::agent::Agent;
use rig::completion::{CompletionModel, GetTokenUsage, Prompt};
use std::env;

use copal::agent::{
    create_gemini_agent, create_ollama_agent, create_openai_agent, default_model, WebFetch,
};
use copal::cli::{render_markdown, run_interactive, Cli};

#[tokio::main]
async fn main() {
    // Load .env file (optional, ignore if not found)
    dotenv().ok();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Cli::parse();

    let provider = env::var("LLM_PROVIDER").unwrap_or_else(|_| "ollama".to_string());
    let model = env::var("LLM_MODEL").unwrap_or_else(|_| default_model(&provider).to_string());
    let web_fetch = WebFetch::new();

    match provider.as_str() {
        "openai" => {
            let api_key =
                env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY required for OpenAI provider");
            let agent = create_openai_agent(&api_key, &model, web_fetch);
            run_with_agent(agent, &args).await;
        }
        "gemini" => {
            let api_key =
                env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY required for Gemini provider");
            let agent = create_gemini_agent(&api_key, &model, web_fetch);
            run_with_agent(agent, &args).await;
        }
        _ => {
            let agent = create_ollama_agent(&model, web_fetch);
            run_with_agent(agent, &args).await;
        }
    }
}

async fn run_with_agent<M>(agent: Agent<M>, args: &Cli)
where
    M: CompletionModel + 'static,
    M::StreamingResponse: GetTokenUsage,
{
    if args.interactive {
        run_interactive(agent).await;
    } else {
        // One-shot mode: require prompt argument
        let prompt = match &args.prompt {
            Some(p) => p,
            None => {
                eprintln!("Error: prompt required. Use -i for interactive mode.");
                return;
            }
        };

        let response = match agent.prompt(prompt).await {
            Ok(r) => r,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };
        render_markdown(&response);
    }
}
