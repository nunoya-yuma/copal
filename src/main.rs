use copal::agent::WebFetch;
use dotenvy::dotenv;

#[cfg(feature = "cli")]
use copal::agent::{create_gemini_agent, create_ollama_agent, create_openai_agent, default_model};
#[cfg(feature = "cli")]
use copal::cli::run_interactive;
#[cfg(feature = "web")]
use copal::{
    agent::AnyAgent,
    web::{build_router, AppState},
};
#[cfg(feature = "web")]
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Load .env file (optional, ignore if not found)
    dotenv().ok();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    #[cfg(feature = "cli")]
    {
        use std::env;

        let provider = env::var("LLM_PROVIDER").unwrap_or_else(|_| "ollama".to_string());
        let model = env::var("LLM_MODEL").unwrap_or_else(|_| default_model(&provider).to_string());
        let web_fetch = WebFetch::new();

        match provider.as_str() {
            "openai" => {
                let api_key = env::var("OPENAI_API_KEY")
                    .expect("OPENAI_API_KEY required for OpenAI provider");
                let agent = create_openai_agent(&api_key, &model, web_fetch);
                run_interactive(agent).await;
            }
            "gemini" => {
                let api_key = env::var("GEMINI_API_KEY")
                    .expect("GEMINI_API_KEY required for Gemini provider");
                let agent = create_gemini_agent(&api_key, &model, web_fetch);
                run_interactive(agent).await;
            }
            _ => {
                let agent = create_ollama_agent(&model, web_fetch);
                run_interactive(agent).await;
            }
        }
    }

    #[cfg(feature = "web")]
    {
        let web_fetch = WebFetch::new();
        let agent = AnyAgent::from_env(web_fetch);
        let app_state = AppState::new(agent);
        let router = build_router(Arc::new(app_state));
        println!("🚀 Server running on http://0.0.0.0:3000");
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .expect("Failed to create listener");
        axum::serve(listener, router)
            .await
            .expect("Failed to start server");
    }

    #[cfg(not(any(feature = "cli", feature = "web")))]
    eprintln!("No feature enabled. Build with: cargo build --features cli or --features web");
}
