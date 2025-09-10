use tonic::{Request, Status};

pub fn check_auth<T>(request: &Request<T>) -> Result<(), Status> {
    // Get API key from metadata - try multiple common formats
    let api_key = request.metadata().get("x-api-key")
        .or_else(|| request.metadata().get("api-key"))
        .or_else(|| request.metadata().get("api_key"))
        .or_else(|| request.metadata().get("apikey"));
    
    // Get expected API key from environment
    let expected_key = std::env::var("API_KEY").unwrap_or_else(|_| "".to_string());
    
    if let Some(key) = api_key {
        if key.to_str().unwrap_or("") == expected_key && !expected_key.is_empty() {
            return Ok(());
        }
    }
    
    Err(Status::unauthenticated("Invalid or missing API key"))
}