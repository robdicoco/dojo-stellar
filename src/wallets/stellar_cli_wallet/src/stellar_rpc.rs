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