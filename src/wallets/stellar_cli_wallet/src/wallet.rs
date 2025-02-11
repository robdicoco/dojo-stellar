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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tokio::runtime::Runtime;
    use std::panic::AssertUnwindSafe;

    // Helper function to create a mock Wallet instance
    fn create_wallet() -> Wallet {
        Wallet::new("https://mock-rpc-url.com")
    }

    #[test]
    fn test_new_wallet() {
        let wallet = create_wallet();
        assert_eq!(wallet.keystore.key_count(), 0); // Use the getter method
    }

    #[test]
    fn test_generate_keypair() {
        let mut wallet = create_wallet();
        wallet.generate_keypair();

        // Ensure one keypair has been added to the keystore
        assert_eq!(wallet.keystore.key_count(), 1);

        // Validate the structure of the keys
        let keys = wallet.keystore.get_keys(); // Use the getter method
        let (public_key, secret_key) = keys.iter().next().unwrap();
        assert!(public_key.starts_with('G')); // Stellar public key starts with 'G'
        assert!(secret_key.starts_with('S')); // Stellar secret key starts with 'S'
    }

    #[test]
    fn test_save_and_load_keystore() {
        let mut wallet = create_wallet();
        wallet.generate_keypair();
    
        // Save the keystore
        wallet.save_keystore("mypassword");
    
        // Load the keystore
        let mut loaded_wallet = create_wallet();
        loaded_wallet.load_keystore("mypassword");
    
        // Verify that the loaded keystore matches the original
        assert_eq!(loaded_wallet.keystore.key_count(), 1);
        assert_eq!(loaded_wallet.keystore.get_keys(), wallet.keystore.get_keys());
    
        // Clean up the test file
        fs::remove_file("keystore.aes").unwrap();
    }

    #[test]
    fn test_invalid_password_load_keystore() {
        let mut wallet = create_wallet();
        wallet.generate_keypair();

        // Save the keystore
        wallet.save_keystore("mypassword");

        // Attempt to load with an incorrect password
        let mut loaded_wallet = create_wallet();
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            loaded_wallet.load_keystore("wrongpassword");
        }));

        // Expect a panic due to decryption failure
        assert!(result.is_err());

        // Clean up the test file
        fs::remove_file("keystore.aes").unwrap();
    }

    #[test]
    fn test_fetch_balance() {
        let wallet = create_wallet();

        // Mock RPC response using Tokio runtime
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let public_key = "GCKFLO7RHFB2HXMEN6EMZPT6WYCIOL3WJPDJWGIKBQA2KAVS6VS4WY4E";
            wallet.fetch_balance(public_key).await;

            // Note: This test assumes the RPC server is mocked or live.
            // For a real test, you'd need to mock the `fetch_account_balance` function.
        });
    }

    #[test]
    fn test_send_xlm() {
        let wallet = create_wallet();

        // Mock RPC response using Tokio runtime
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let source_secret = "SB3KUBH6VZ7ZL7ZVJ4XUQY5Z6Z7ZL7ZVJ4XUQY5Z6Z7ZL7ZVJ4XUQY5Z6";
            let destination_public_key = "GCKFLO7RHFB2HXMEN6EMZPT6WYCIOL3WJPDJWGIKBQA2KAVS6VS4WY4E";
            let amount = "10";

            wallet
                .send_xlm(source_secret, destination_public_key, amount)
                .await;

            // Note: This test assumes the RPC server is mocked or live.
            // For a real test, you'd need to mock the `send_transaction` function.
        });
    }
}