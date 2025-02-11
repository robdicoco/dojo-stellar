use reqwest::Client;
use serde_json::Value;

pub async fn fetch_account_balance(rpc_url: &str, public_key: &str) -> Result<Vec<Value>, String> {
    let client = Client::new();
    let url = format!("{}/accounts/{}", rpc_url, public_key);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch account: {}", response.status()));
    }
    let account: Value = response.json().await.map_err(|e| e.to_string())?;
    Ok(account["balances"].as_array().unwrap_or(&vec![]).to_vec())
}

pub async fn send_transaction(
    rpc_url: &str,
    source_secret: &str,
    destination_public_key: &str,
    amount: &str,
) -> Result<Value, String> {
    let client = Client::new();
    let url = format!("{}/transactions", rpc_url);

    // Construct the transaction payload (XDR format or JSON depending on API)
    let transaction_payload = serde_json::json!({
        "source_account": source_secret,
        "operations": [
            {
                "type": "payment",
                "destination": destination_public_key,
                "amount": amount,
                "asset": {
                    "type": "native"
                }
            }
        ]
    });

    let response = client.post(&url).json(&transaction_payload).send().await.map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("Failed to submit transaction: {}", response.status()));
    }
    let result: Value = response.json().await.map_err(|e| e.to_string())?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};
    use serde_json::json;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_fetch_account_balance_success() {
        // Start a mock server
        let mock_server = MockServer::start().await;

        let public_key = "GCKFLO7RHFB2HXMEN6EMZPT6WYCIOL3WJPDJWGIKBQA2KAVS6VS4WY4E";

        // Define the mock response
        let mock_response = json!({
            "balances": [
                {"balance": "100.0000000", "asset_type": "native"},
                {"balance": "50.0000000", "asset_code": "USD", "asset_issuer": "issuer"}
            ]
        });

        // Mount the mock on the mock server
        Mock::given(method("GET"))
            .and(path(format!("/accounts/{}", public_key)))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
            .mount(&mock_server)
            .await;

        // Use the mock server's URL for the RPC call
        let rpc_url = mock_server.uri();
        let result = fetch_account_balance(&rpc_url, public_key).await;

        // Verify the result
        assert!(result.is_ok());
        let balances = result.unwrap();
        assert_eq!(balances.len(), 2);
        assert_eq!(balances[0]["balance"], "100.0000000");
        assert_eq!(balances[1]["balance"], "50.0000000");
    }

    #[tokio::test]
    #[serial]
    async fn test_fetch_account_balance_failure() {
        // Start a mock server
        let mock_server = MockServer::start().await;

        let public_key = "GCKFLO7RHFB2HXMEN6EMZPT6WYCIOL3WJPDJWGIKBQA2KAVS6VS4WY4E";

        // Define the mock response with an error
        Mock::given(method("GET"))
            .and(path(format!("/accounts/{}", public_key)))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"error": "Account not found"})))
            .mount(&mock_server)
            .await;

        // Use the mock server's URL for the RPC call
        let rpc_url = mock_server.uri();
        let result = fetch_account_balance(&rpc_url, public_key).await;

        // Verify the result
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Failed to fetch account: 404 Not Found");
    }

    #[tokio::test]
    #[serial]
    async fn test_send_transaction_success() {
        // Start a mock server
        let mock_server = MockServer::start().await;

        let source_secret = "SB3KUBH6VZ7ZL7ZVJ4XUQY5Z6Z7ZL7ZVJ4XUQY5Z6Z7ZL7ZVJ4XUQY5Z6";
        let destination_public_key = "GCKFLO7RHFB2HXMEN6EMZPT6WYCIOL3WJPDJWGIKBQA2KAVS6VS4WY4E";
        let amount = "10";

        // Define the mock response
        let mock_response = json!({
            "hash": "abcdef1234567890",
            "successful": true
        });

        // Mount the mock on the mock server
        Mock::given(method("POST"))
            .and(path("/transactions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
            .mount(&mock_server)
            .await;

        // Use the mock server's URL for the RPC call
        let rpc_url = mock_server.uri();
        let result = send_transaction(&rpc_url, source_secret, destination_public_key, amount).await;

        // Verify the result
        assert!(result.is_ok());
        let transaction_result = result.unwrap();
        assert_eq!(transaction_result["hash"], "abcdef1234567890");
        assert_eq!(transaction_result["successful"], true);
    }

    #[tokio::test]
    #[serial]
    async fn test_send_transaction_failure() {
        // Start a mock server
        let mock_server = MockServer::start().await;

        let source_secret = "SB3KUBH6VZ7ZL7ZVJ4XUQY5Z6Z7ZL7ZVJ4XUQY5Z6Z7ZL7ZVJ4XUQY5Z6";
        let destination_public_key = "GCKFLO7RHFB2HXMEN6EMZPT6WYCIOL3WJPDJWGIKBQA2KAVS6VS4WY4E";
        let amount = "10";

        // Define the mock response with an error
        Mock::given(method("POST"))
            .and(path("/transactions"))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({"error": "Invalid transaction"})))
            .mount(&mock_server)
            .await;

        // Use the mock server's URL for the RPC call
        let rpc_url = mock_server.uri();
        let result = send_transaction(&rpc_url, source_secret, destination_public_key, amount).await;

        // Verify the result
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Failed to submit transaction: 400 Bad Request");
    }
}