use crate::web::{handlers::chat_handler, AppState};
use axum::{routing::post, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

/// Build the Axum router with all routes and middleware
///
/// # Routes
/// - POST /api/chat - SSE streaming chat endpoint
/// - GET / - Serve static files from frontend/dist (production mode)
///
/// # Middleware
/// - CORS: Allow all origins (for development)
///
/// # Static File Serving
/// The router serves static files from `frontend/dist/` for production deployment.
/// API routes are matched first, so `/api/*` requests go to handlers, while all
/// other requests serve files from the dist directory.
pub fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/chat", post(chat_handler))
        .fallback_service(ServeDir::new("frontend/dist"))
        .with_state(state)
        .layer(CorsLayer::permissive())
}
