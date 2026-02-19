/// Configuration for API endpoints
///
/// In development (trunk serve), uses relative path which gets proxied by Trunk
/// In production (release build), uses the full API domain

/// Get the base URL for API calls
pub fn api_base_url() -> &'static str {
    #[cfg(debug_assertions)]
    {
        // Development: use relative path (Trunk proxies to localhost:8787)
        "/api"
    }

    #[cfg(not(debug_assertions))]
    {
        // Production: use full API domain
        "https://inteligencia-animal-api.cgutieco.com/api"
    }
}

