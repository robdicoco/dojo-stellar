mod wallet;
mod keystore;
mod stellar_rpc;
mod utils;

use wallet::Wallet;
use std::io::{self, Write};
use rpassword::read_password;

#[tokio::main]
async fn main() {
    let mut wallet = Wallet::new("https://horizon-testnet.stellar.org");

    loop {
        println!("\nStellar Wallet CLI");
        println!("1. Generate New Keypair");
        println!("2. Fetch Balance");
        println!("3. Send XLM");
        println!("4. Save Keystore");
        println!("5. Load Keystore");
        println!("6. Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => wallet.generate_keypair(),
            "2" => {
                print!("Enter Public Key: ");
                io::stdout().flush().unwrap();
                let mut public_key = String::new();
                io::stdin().read_line(&mut public_key).unwrap();
                wallet.fetch_balance(public_key.trim()).await;
            }
            "3" => {
                print!("Enter Source Secret Key: ");
                io::stdout().flush().unwrap();
                let mut source_secret = String::new();
                io::stdin().read_line(&mut source_secret).unwrap();

                print!("Enter Destination Public Key: ");
                io::stdout().flush().unwrap();
                let mut destination_public_key = String::new();
                io::stdin().read_line(&mut destination_public_key).unwrap();

                print!("Enter Amount: ");
                io::stdout().flush().unwrap();
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).unwrap();

                wallet
                    .send_xlm(
                        source_secret.trim(),
                        destination_public_key.trim(),
                        amount.trim(),
                    )
                    .await;
            }
            "4" => {
                print!("Enter Password: ");
                io::stdout().flush().unwrap();
                let password = read_password().unwrap();
                wallet.save_keystore(&password);
            }
            "5" => {
                print!("Enter Password: ");
                io::stdout().flush().unwrap();
                let password = read_password().unwrap();
                wallet.load_keystore(&password);
            }
            "6" => break,
            _ => println!("Invalid option!"),
        }
    }
}