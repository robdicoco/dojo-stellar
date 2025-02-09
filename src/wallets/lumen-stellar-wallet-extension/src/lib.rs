// mod utils;

use wasm_bindgen::prelude::*;
use web_sys::window;
use serde::{Serialize, Deserialize};
use aes::Aes128;
use block_modes::{BlockMode, Cfb};
use rand::Rng;
use reqwest::Client;
use tokio;


type Aes128Cfb = Cfb<Aes128>;

#[derive(Serialize, Deserialize)]
struct EncryptedKeystore {
    iv: String,
    data: String,
}


#[wasm_bindgen]
pub fn encrypt_data(password: &str, secret_seed: &str) -> String {
    let key = password.as_bytes();
    let mut iv = [0u8; 16];
    rand::thread_rng().fill(&mut iv);
    let cipher = Aes128Cfb::new_from_slices(key, &iv).unwrap();
    let encrypted_data = cipher.encrypt_vec(secret_seed.as_bytes());
    let keystore = EncryptedKeystore {
        iv: hex::encode(iv),
        data: hex::encode(encrypted_data),
    };
    serde_json::to_string(&keystore).unwrap()
}

#[wasm_bindgen]
pub fn decrypt_data(password: &str, keystore_json: &str) -> String {
    let key = password.as_bytes();
    let keystore: EncryptedKeystore = serde_json::from_str(keystore_json).unwrap();
    let iv = hex::decode(&keystore.iv).unwrap();
    let encrypted_data = hex::decode(&keystore.data).unwrap();
    let cipher = Aes128Cfb::new_from_slices(key, &iv).unwrap();
    let decrypted_data = cipher.decrypt_vec(&encrypted_data).unwrap();
    String::from_utf8(decrypted_data).unwrap()
}


#[wasm_bindgen]
pub async fn fetch_balance(public_key: &str, rpc_url: &str) -> Result<String, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");

    let url = format!("{}/accounts/{}", rpc_url, public_key);
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp: Response = JsFuture::from(window.fetch_with_request(&request)).await?.dyn_into()?;

    let json = JsFuture::from(resp.json()?).await?;
    let balance = js_sys::Reflect::get(&json, &JsValue::from_str("balances"))
        .unwrap()
        .unchecked_into::<js_sys::Array>()
        .get(0)
        .as_object()
        .unwrap()
        .get("balance")
        .as_string()
        .unwrap_or_else(|| "0".to_string());

    Ok(balance)
}

#[wasm_bindgen]
pub fn create_keypair() -> String {
    let keypair = stellar_sdk::Keypair::random();
    let public_key = keypair.public_key();
    let secret_seed = keypair.secret_seed().unwrap();
    serde_json::json!({
        "publicKey": public_key,
        "secretSeed": secret_seed
    })
    .to_string()
}
// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, lumen-stellar-wallet-extension!");
// }
