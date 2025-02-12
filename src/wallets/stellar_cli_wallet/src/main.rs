mod wallet;
mod keystore;
mod stellar_rpc;
mod utils;

use wallet::Wallet;
// use keystore::{Identity};
use std::io::{self, Write};
use rpassword::read_password;
use utils::{fund_soroban_address, get_api_key};

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
        println!("6. Fund Wallet");
        // println!("7. Generate Identity");
        // println!("8. Load Identity");
        println!("9. Exit");

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
            "6" => {
                print!("Enter Public Key to fund: ");
                io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately
                let mut soroban_address = String::new();
                io::stdin()
                    .read_line(&mut soroban_address)
                    .expect("Failed to read input");

                let soroban_address = soroban_address.trim();
                
                // Use testnet friendbot URL
                let friendbot_url = "https://friendbot.stellar.org";
                
                // Use the local RPC URL with the /friendbot endpoint
                let local_rpc_url = "https://lumen.758206.xyz:8001/soroban-rpc";

                // Retrieve the API key from the environment variable
                let api_key_result = get_api_key();
                let api_key_string; // Store the String here to extend its lifetime
                let api_key = match api_key_result {
                    Ok(key) => {
                        api_key_string = key; // Store the String in a variable
                        Some(api_key_string.as_str()) // Create a reference to the stored String
                    }
                    Err(_) => {
                        eprintln!("Warning: LOCAL_RPC_API_KEY not set. Proceeding without API key.");
                        None // No API key available
                    }
                };
                
                // Call the fund_soroban_address function
                match fund_soroban_address(
                    soroban_address,
                    api_key, // Pass as Option<&str>
                    friendbot_url,
                    local_rpc_url,
                    None, // Default to funding both networks
                )
                .await
                {
                    Ok(_) => println!("Funding successful!"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            // "7" => {
            //     print!("Enter Identity Name: ");
            //     io::stdout().flush().unwrap();
            //     let mut name = String::new();
            //     io::stdin().read_line(&mut name).unwrap();
            //     let name = name.trim();

            //     print!("Enter Seed (optional, press Enter to generate random): ");
            //     io::stdout().flush().unwrap();
            //     let mut seed = String::new();
            //     io::stdin().read_line(&mut seed).unwrap();
            //     let seed = if seed.trim().is_empty() { None } else { Some(seed.trim().to_string()) };

            //     let (seed_phrase, keypair) = Identity::generate(name.to_string(), seed);
            //     println!("Generated Seed Phrase: {}", seed_phrase);
            //     println!("Generated Public Key: {}", keypair.public_key());

            //     print!("Save identity? (y/n): ");
            //     io::stdout().flush().unwrap();
            //     let mut save_choice = String::new();
            //     io::stdin().read_line(&mut save_choice).unwrap();

            //     if save_choice.trim().eq_ignore_ascii_case("y") {
            //         print!("Enter Password: ");
            //         io::stdout().flush().unwrap();
            //         let password = read_password().unwrap();

            //         let identity = Identity {
            //             name: name.to_string(),
            //             seed_phrase: seed_phrase.to_string(),
            //         };

            //         identity.save(&password).unwrap();
            //         println!("Identity saved successfully!");
            //     }
            // }
            // "8" => {
            //     print!("Enter Identity Name: ");
            //     io::stdout().flush().unwrap();
            //     let mut name = String::new();
            //     io::stdin().read_line(&mut name).unwrap();
            //     let name = name.trim();

            //     print!("Enter Password: ");
            //     io::stdout().flush().unwrap();
            //     let password = read_password().unwrap();

            //     let identity = Identity::load(name, &password).unwrap();
            //     let mnemonic = bip39::Mnemonic::from_phrase(&identity.seed_phrase, bip39::Language::English).unwrap();
            //     let seed_bytes = bip39::Seed::new(&mnemonic, "").as_bytes();
            //     let keypair = stellar_sdk::types::KeyPair::from_seed(&seed_bytes[..32]).unwrap();

            //     println!("Loaded Public Key: {}", keypair.public_key());
            // }
            "9" => break,
            _ => println!("Invalid option!"),
        }
    }
}