use reqwest::Client;
use serde_json::json;
use std::error::Error;
use std::env;

/// Fund a Soroban address using Stellar Friendbot on testnet and local RPC.
///
/// # Arguments
/// * `soroban_address` - The Soroban address to fund.
/// * `api_key` - The API key for the local RPC server.
/// * `friendbot_url` - The URL of the Stellar Friendbot.
/// * `local_rpc_url` - The URL of the local RPC server.
///
/// # Returns
/// A `Result` indicating success or failure.
pub async fn fund_soroban_address(
    soroban_address: &str,
    api_key: &str,
    friendbot_url: &str,
    local_rpc_url: &str,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    
    // Call Friendbot
    let friendbot_endpoint = format!("{}/?addr={}", friendbot_url, soroban_address);
    let response = client.get(&friendbot_endpoint).send().await?;
    
    if !response.status().is_success() {
        return Err(format!("Friendbot funding failed: {}", response.status()).into());
    }

    // Call local RPC
    let rpc_payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "fund_account",
        "params": {"address": soroban_address}
    });
    
    let response = client
        .post(local_rpc_url)
        .header("api-key", api_key)
        .json(&rpc_payload)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("RPC funding failed: {}", response.status()).into());
    }

    Ok(())
}

/// Reads the API key from environment variable
pub fn get_api_key() -> Result<String, Box<dyn Error>> {
    env::var("LOCAL_RPC_API_KEY")
        .map_err(|_| "LOCAL_RPC_API_KEY not set in environment".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path, query_param, header};

    const TEST_ADDRESS: &str = "GCKFLO7RHFB2HXMEN6EMZPT6WYCIOL3WJPDJWGIKBQA2KAVS6VS4WY4E";
    const API_KEY: &str = "test_api_key";

    async fn setup_mock_server() -> MockServer {
        MockServer::start().await
    }

    #[tokio::test]
    async fn test_successful_funding() {
        let mock_server = setup_mock_server().await;

        // Friendbot mock
        Mock::given(method("GET"))
            .and(path("/"))
            .and(query_param("addr", TEST_ADDRESS))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        // RPC mock
        Mock::given(method("POST"))
            .and(path("/"))
            .and(header("api-key", API_KEY))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let result = fund_soroban_address(
            TEST_ADDRESS,
            API_KEY,
            &mock_server.uri(),
            &mock_server.uri()
        ).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_friendbot_failure() {
        let mock_server = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/"))
            .and(query_param("addr", TEST_ADDRESS))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let result = fund_soroban_address(
            TEST_ADDRESS,
            API_KEY,
            &mock_server.uri(),
            "http://dummy-url"
        ).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("500"));
    }

    #[tokio::test]
    async fn test_rpc_failure() {
        let mock_server = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/"))
            .and(query_param("addr", TEST_ADDRESS))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        Mock::given(method("POST"))
            .and(path("/"))
            .and(header("api-key", API_KEY))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let result = fund_soroban_address(
            TEST_ADDRESS,
            API_KEY,
            &mock_server.uri(),
            &mock_server.uri()
        ).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("403"));
    }

    #[test]
    fn test_api_key_handling() {
        // Test valid API key
        env::set_var("LOCAL_RPC_API_KEY", "valid_key");
        assert_eq!(get_api_key().unwrap(), "valid_key");
        
        // Test missing API key
        env::remove_var("LOCAL_RPC_API_KEY");
        assert!(get_api_key().is_err());
    }
}