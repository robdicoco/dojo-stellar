use crate::keystore::Keystore;
use crate::stellar_rpc::{fetch_account_balance, send_transaction};
use std::fs;

pub struct Wallet {
    keystore: Keystore,
    rpc_url: String,
}

impl Wallet {
    pub fn new(rpc_url: &str) -> Self {
        Wallet {
            keystore: Keystore::new(),
            rpc_url: rpc_url.to_string(),
        }
    }

    pub fn generate_keypair(&mut self) {
        let (public_key, secret_key) = Keystore::generate_keypair();
        self.keystore.add_key(public_key.clone(), secret_key.clone());
        println!("New Keypair Generated:");
        println!("Public Key: {}", public_key);
        println!("Secret Key: {}", secret_key);
    }

    pub async fn fetch_balance(&self, public_key: &str) {
        match fetch_account_balance(&self.rpc_url, public_key).await {
            Ok(balances) => {
                println!("Balances for {}:", public_key);
                for balance in balances {
                    println!("{:?}", balance);
                }
            }
            Err(e) => println!("Error fetching balance: {}", e),
        }
    }

    pub async fn send_xlm(&self, source_secret: &str, destination_public_key: &str, amount: &str) {
        match send_transaction(
            &self.rpc_url,
            source_secret,
            destination_public_key,
            amount,
        )
        .await
        {
            Ok(response) => println!("Transaction Successful: {:?}", response),
            Err(e) => println!("Error sending transaction: {}", e),
        }
    }

    pub fn save_keystore(&self, password: &str) {
        let encrypted = self.keystore.encrypt(password);
        fs::write("keystore.aes", encrypted).unwrap();
        println!("Keystore saved securely.");
    }

    pub fn load_keystore(&mut self, password: &str) {
        let encrypted = fs::read_to_string("keystore.aes").unwrap();
        self.keystore = Keystore::decrypt(password, &encrypted);
        println!("Keystore loaded.");
    }
}