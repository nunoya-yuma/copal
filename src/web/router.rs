use crate::web::{handlers::chat_handler, AppState};
use axum::{routing::post, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

/// Build the Axum router with all routes and middleware
///
/// # Routes
/// - POST /api/chat - SSE streaming chat endpoint
///
/// # Middleware
/// - CORS: Allow all origins (for development)
pub fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/chat", post(chat_handler))
        .with_state(state)
        .layer(CorsLayer::permissive())
}
