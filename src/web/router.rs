// TODO(human): You'll need these for implementation
#[allow(unused_imports)]
use axum::{routing::post, Router};
use std::sync::Arc;
#[allow(unused_imports)]
use tower_http::cors::CorsLayer;

#[allow(unused_imports)]
use crate::web::{handlers::chat_handler, AppState};

/// Build the Axum router with all routes and middleware
///
/// # Routes
/// - POST /api/chat - SSE streaming chat endpoint
///
/// # Middleware
/// - CORS: Allow all origins (for development)
///
/// # TODO(human): Implement build_router
/// 1. Create a Router with state (AppState)
/// 2. Add POST /api/chat route pointing to chat_handler
/// 3. Add CorsLayer::permissive() middleware
/// 4. Return the configured router
pub fn build_router(_state: Arc<AppState>) -> Router {
    // TODO(human): Implement build_router
    // For now, return an empty router to satisfy the compiler
    Router::new()
}
