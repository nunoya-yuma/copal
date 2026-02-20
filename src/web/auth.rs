use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::web::AppState;

/// Axum middleware that validates the Bearer token in the Authorization header.
///
/// # Flow
/// 1. Extract the `Authorization` header from the request
/// 2. Parse the Bearer token from the header value
/// 3. Compare the token against `AppState.api_token`
/// 4. If valid: pass the request to the next handler
/// 5. If invalid or missing: return 401 Unauthorized
///
/// # Usage
/// Applied via `.route_layer()` in the router so that only API routes
/// are protected (static file serving remains open).
pub async fn require_bearer_token(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .get("authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if token != state.api_token {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::require_bearer_token;
    use crate::agent::{AnyAgent, WebFetch};
    use crate::web::AppState;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        middleware,
        routing::post,
        Router,
    };
    use std::sync::Arc;
    use tower::ServiceExt;

    fn test_router(token: &str) -> Router {
        let state = Arc::new(AppState::new(
            AnyAgent::from_env(WebFetch::new()),
            token.to_string(),
        ));
        Router::new()
            .route("/test", post(|| async { "ok" }))
            .route_layer(middleware::from_fn_with_state(
                Arc::clone(&state),
                require_bearer_token,
            ))
            .with_state(state)
    }

    #[tokio::test]
    async fn test_allows_request_with_correct_token() {
        let response = test_router("test-token")
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/test")
                    .header("authorization", "Bearer test-token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_rejects_request_without_token() {
        let response = test_router("test-token")
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_rejects_request_with_wrong_token() {
        let response = test_router("test-token")
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/test")
                    .header("authorization", "Bearer wrong-token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
