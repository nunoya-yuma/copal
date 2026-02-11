use dotenvy::dotenv;

#[cfg(feature = "cli")]
use copal::agent::{
    create_gemini_agent, create_ollama_agent, create_openai_agent, default_model, WebFetch,
};
#[cfg(feature = "cli")]
use copal::cli::run_interactive;

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
        // TODO(human): Implement web server startup
        // 1. Import necessary modules (AnyAgent, AppState, build_router)
        // 2. Get provider and model from environment variables
        // 3. Create AnyAgent using AnyAgent::from_env(WebFetch::new())
        // 4. Create AppState with the agent
        // 5. Build router with build_router(state)
        // 6. Create TcpListener on 0.0.0.0:3000
        // 7. Start server with axum::serve(listener, router.into_make_service()).await
        // 8. Add proper error handling and logging

        eprintln!("Web server startup not yet implemented");
    }

    #[cfg(not(any(feature = "cli", feature = "web")))]
    eprintln!("No feature enabled. Build with: cargo build --features cli or --features web");
}
