use crate::web::{auth::require_bearer_token, handlers::chat_handler, AppState};
use axum::{middleware, routing::post, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

/// Build the Axum router with all routes and middleware
///
/// # Routes
/// - POST /api/chat - SSE streaming chat endpoint (Bearer token required)
/// - GET / - Serve static files from frontend/dist (no auth required)
///
/// # Middleware
/// - Auth: Bearer token validation applied via `.route_layer()` (API routes only)
/// - CORS: Allow all origins (for development)
///
/// # Why `.route_layer()` instead of `.layer()`
/// `.layer()` wraps the entire router including the ServeDir fallback, which would
/// require a token just to load `index.html`. `.route_layer()` applies only to
/// explicitly registered routes (`/api/chat`), leaving static file serving open.
pub fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/chat", post(chat_handler))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            require_bearer_token,
        ))
        .fallback_service(ServeDir::new("frontend/dist"))
        .with_state(state)
        .layer(CorsLayer::permissive())
}
