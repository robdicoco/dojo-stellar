use reqwest::Client;
use std::error::Error;
use std::env;

/// Fund a Soroban address using Stellar Friendbot on testnet and local RPC.
///
/// # Arguments
/// * `soroban_address` - The Soroban address to fund.
/// * `api_key` - The API key for the local RPC server.
/// * `friendbot_url` - The URL of the Stellar Friendbot.
/// * `local_rpc_url` - The URL of the local RPC server.
/// * `network` - The network to fund: "L" for Local, "T" for TestNet, "B" for Both (default is "B").
///
/// # Returns
/// A `Result` indicating success or failure.
pub async fn fund_soroban_address(
    soroban_address: &str,
    api_key: Option<&str>,
    friendbot_url: &str,
    local_rpc_url: &str,
    network: Option<&str>, // Optional parameter to select network
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let network = network.unwrap_or("B"); // Default to "B" (Both)

    // Helper function to handle HTTP errors
    async fn handle_response(response: reqwest::Response, context: &str) -> Result<(), Box<dyn Error>> {
        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_details = if status == 400 {
                response.text().await.unwrap_or_else(|_| "No additional details".to_string())
            } else {
                "No additional details".to_string()
            };
            Err(format!("{} failed: {} - {}", context, status, error_details).into())
        }
    }

    if network == "T" || network == "B" {
        // Call Friendbot (TestNet)
        let friendbot_endpoint = format!("{}/?addr={}", friendbot_url, soroban_address);
        let response = client.get(&friendbot_endpoint).send().await?;
        handle_response(response, "Friendbot funding").await?;
    }

    if network == "L" || network == "B" {
        // Call local RPC Friendbot
        let local_friendbot_endpoint = format!(
            "{}/friendbot?addr={}",
            local_rpc_url.trim_end_matches('/'), // Ensure no trailing slash
            soroban_address
        );

        let mut request = client.get(&local_friendbot_endpoint);

        // Add API key header if provided
        if let Some(key) = api_key {
            request = request.header("api-key", key);
        }

        let response = request.send().await?;
        handle_response(response, "Local RPC funding").await?;
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
            Some(API_KEY), 
            &mock_server.uri(),
            &mock_server.uri(),
            None
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
            Some(API_KEY), 
            &mock_server.uri(),
            "http://dummy-url",
            None
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
            Some(API_KEY), 
            &mock_server.uri(),
            &mock_server.uri(),
            None
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