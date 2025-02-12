[![🇺🇸 English](https://img.shields.io/badge/Lang-EN-blue)](./challenge2-en.md)
[![🇧🇷 Portuguese](https://img.shields.io/badge/Lang-PT--BR-green)](./challenge2.md)

[← Back to README](../README.md)

# Dojo Stellar - Team Lumen League ✨

<p align="center">
  <img src="./letreiro_lumen_wallet-en.gif" alt="Lumen League Logo" width="1000">
</p>

[![Status](https://img.shields.io/badge/Status-Concluded-green)](#)
![Stellar](https://img.shields.io/badge/Stellar-Blockchain-blue)
![License](https://img.shields.io/badge/License-MIT-green)

## 📜 Description
The **Lumen Stellar CLI Wallet** is a command-line interface (CLI) tool developed to simplify interaction with the **Stellar blockchain** and **Soroban RPC** services. It is ideal for developers and users who want to manage Stellar accounts, test smart contracts, or perform transactions easily and securely. This project is part of the **Dojo Stellar – Week 2** program.

## 🚀 Objective
Provide a robust and efficient command-line interface for:
- Generating public/private key pairs.
- Checking Stellar account balances.
- Performing secure XLM transactions.
- Managing keystores for secure storage of private keys.
- Funding testnet wallets using Friendbot.

## ⚙️ Features
- **Key Generation**: Create new public/private key pairs.  
- **Balance Check**: Quickly check the balance of any Stellar account.  
- **Send XLM**: Secure transactions between Stellar accounts.  
- **Keystore Management**: Store and retrieve encrypted private keys.  
- **Wallet Funding**: Use Friendbot to fund testnet accounts.  
- **Environment Integration**: Support for local RPC API configuration.

## 📅 Timeline
- **Start:** 02/05/2025  
- **Delivery:** 02/12/2025  

---

## 🛠️ Technologies Used
- **Language:** Rust  
- **Blockchain:** Stellar  
- **RPC Service:** Soroban  

---

## 📦 Installation
To install the **Lumen Stellar CLI Wallet**, follow these steps:

1. Clone the repository:  
   ```bash
   git clone <repository_url>
   cd <repository_directory>
   ```

2. Build the project:  
   ```bash
   cargo build --release
   ```

3. Run the CLI:  
   ```bash
   ./target/release/stellar_wallet_cli
   ```

---

## 📚 Usage
After running the CLI, you can perform the following operations:

1. **Generate New Keypair**  
   ```bash
   Generate New Keypair
   ```

2. **Fetch Balance**  
   ```bash
   Fetch Balance
   Enter Public Key: <public_key>
   ```

3. **Send XLM**  
   ```bash
   Send XLM
   Enter Source Secret Key: <source_secret_key>
   Enter Destination Public Key: <destination_public_key>
   Enter Amount: <amount>
   ```

4. **Save Keystore**  
   ```bash
   Save Keystore
   Enter Password: <password>
   ```

5. **Load Keystore**  
   ```bash
   Load Keystore
   Enter Password: <password>
   ```

6. **Fund Wallet (Testnet)**  
   ```bash
   Fund Wallet
   Enter Public Key to fund: <public_key>
   ```

7. **Exit**  
   ```bash
   Exit
   ```

---

## 🤝 Contribution
Contributions are welcome! If you want to contribute to this project, follow these steps:

1. Fork the repository.  
2. Create a branch for your feature or bugfix.  
3. Implement your changes.  
4. Commit and push your changes.  
5. Create a pull request.  

### Code Style
- Follow **Rust's** idiomatic style guidelines.  
- Format your code using `cargo fmt` before submitting a PR.

---

## 📄 License
This project is licensed under the **MIT License**. See the [LICENSE](./LICENSE) file for details.

---

<p align="center">🌟 Developed during the <strong>Dojo Stellar</strong> program – Team <strong>Lumen League</strong> 🚀</p>
