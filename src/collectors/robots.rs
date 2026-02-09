use std::collections::HashMap;

use log::{debug, warn};
use reqwest::Url;
use std::sync::Arc;
use texting_robots::{get_robots_url, Robot};
use tokio::sync::Mutex;

use super::web::{HttpClient, USER_AGENT};

/// Cache for robots.txt per domain origin.
/// Stores parsed `Robot` instances keyed by origin (e.g. "https://example.com").
/// Uses `Arc<Mutex<...>>` so clones share the same cache (e.g. across Web server requests).
#[derive(Clone)]
pub(crate) struct RobotsCache {
    cache: Arc<Mutex<HashMap<String, Option<Robot>>>>,
}

impl RobotsCache {
    pub(crate) fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Check if the given URL is allowed by the site's robots.txt.
    /// Returns `true` (allow) on fetch/parse errors (graceful fallback).
    pub(crate) async fn is_allowed<C: HttpClient>(&self, client: &C, url: &str) -> bool {
        // http://exmaple.com/somethig/... -> http://exmaple.com
        let extracted_url = match extract_origin(url) {
            Some(u) => u,
            None => return true,
        };

        // Check whether this URL has already been visited
        {
            let locked_cache = self.cache.lock().await;
            match locked_cache.get(&extracted_url) {
                Some(Some(r)) => return r.allowed(url),
                Some(None) => return true,
                None => {}
            };
        }

        let robots_url = match get_robots_url(&extracted_url) {
            Ok(u) => u,
            Err(e) => {
                warn!("Failed to generate a path to robots.txt: {}", e);
                return true;
            }
        };

        // Download robots.txt from URL
        let robot_txt = match client.get(&robots_url).await {
            Ok(r) => r,
            Err(e) => {
                debug!("Failed to get robots.txt: {}", e);
                let mut locked_cache = self.cache.lock().await;
                locked_cache.insert(extracted_url, None);
                return true;
            }
        };

        // Build the Robot for our friendly User-Agent
        let result: bool;
        {
            let mut locked_cache = self.cache.lock().await;
            let robot = match Robot::new(USER_AGENT, robot_txt.as_bytes()) {
                Ok(r) => r,
                Err(e) => {
                    warn!("robots.txt might be invalid: {}", e);
                    locked_cache.insert(extracted_url, None);
                    return true;
                }
            };
            result = robot.allowed(url);
            locked_cache.insert(extracted_url, Some(robot));
        }

        result
    }
}

/// Extract the origin (scheme + host + port) from a URL.
/// e.g. "https://example.com/path" -> "https://example.com"
pub(crate) fn extract_origin(url: &str) -> Option<String> {
    // Parse the URL and return scheme + host (+ port if non-default)
    let mut parsed_url = match Url::parse(url) {
        Ok(u) => u,
        Err(e) => {
            warn!("Invalid URL: {}", e);
            return None;
        }
    };
    parsed_url.set_path("");
    parsed_url.set_query(None);
    parsed_url.set_fragment(None);

    let result = parsed_url.to_string().trim_end_matches('/').to_string();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    struct MockHttpClient {
        responses: HashMap<String, String>,
    }

    impl MockHttpClient {
        fn new() -> Self {
            Self {
                responses: HashMap::new(),
            }
        }

        fn with_response(mut self, url: &str, body: &str) -> Self {
            self.responses.insert(url.to_string(), body.to_string());
            self
        }
    }

    impl HttpClient for MockHttpClient {
        async fn get(&self, url: &str) -> Result<String> {
            self.responses
                .get(url)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("No mock response for {}", url))
        }
    }

    // --- extract_origin tests ---

    #[test]
    fn test_extract_origin_https() {
        assert_eq!(
            extract_origin("https://example.com/path/to/page"),
            Some("https://example.com".to_string())
        );
    }

    #[test]
    fn test_extract_origin_with_port() {
        assert_eq!(
            extract_origin("http://localhost:8080/api"),
            Some("http://localhost:8080".to_string())
        );
    }

    #[test]
    fn test_extract_origin_invalid_url() {
        assert_eq!(extract_origin("not-a-url"), None);
    }

    // --- is_allowed tests ---

    #[tokio::test]
    async fn test_allowed_when_robots_txt_permits() {
        let client = MockHttpClient::new()
            .with_response("https://example.com/robots.txt", "User-agent: *\nAllow: /");
        let cache = RobotsCache::new();

        assert!(cache.is_allowed(&client, "https://example.com/page").await);
    }

    #[tokio::test]
    async fn test_blocked_when_robots_txt_disallows() {
        let client = MockHttpClient::new().with_response(
            "https://example.com/robots.txt",
            "User-agent: *\nDisallow: /secret",
        );
        let cache = RobotsCache::new();

        assert!(
            !cache
                .is_allowed(&client, "https://example.com/secret/page")
                .await
        );
    }

    #[tokio::test]
    async fn test_allowed_when_robots_txt_fetch_fails() {
        // No mock response for robots.txt => fetch fails => graceful fallback to allow
        let client = MockHttpClient::new();
        let cache = RobotsCache::new();

        assert!(cache.is_allowed(&client, "https://example.com/page").await);
    }

    #[tokio::test]
    async fn test_single_cache_is_shared() {
        let clone_a = RobotsCache::new();
        let clone_b = clone_a.clone();
        let client = MockHttpClient::new()
            .with_response("https://example.com/robots.txt", "User-agent: *\nAllow: /");

        clone_a
            .is_allowed(&client, "https://example.com/page")
            .await;

        let locked = clone_b.cache.lock().await;
        assert_eq!(locked.len(), 1)
    }

    #[tokio::test]
    async fn test_cache_reuses_robots_txt_for_same_origin() {
        let client = MockHttpClient::new().with_response(
            "https://example.com/robots.txt",
            "User-agent: *\nDisallow: /blocked",
        );
        let cache = RobotsCache::new();

        // First call fetches robots.txt
        assert!(cache.is_allowed(&client, "https://example.com/ok").await);
        // Second call should reuse cached robots.txt (same origin)
        assert!(
            !cache
                .is_allowed(&client, "https://example.com/blocked/page")
                .await
        );
        // Only one entry in cache
        let locked_cache = cache.cache.lock().await;
        assert_eq!(locked_cache.len(), 1);
    }
}
