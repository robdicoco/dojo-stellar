# Lumen Stellar CLI Wallet

[![Crates.io](https://img.shields.io/crates/v/stellar_cli_wallet_lumen.svg)](https://crates.io/crates/stellar_cli_wallet) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://chat.qwenlm.ai/c/LICENSE)

A command-line interface (CLI) tool for interacting with the Stellar network and Soroban RPC. This tool allows you to generate keypairs, fetch balances, send XLM, fund wallets, and manage keystores securely.

## Table of Contents

1.  [Overview](https://chat.qwenlm.ai/c/a85eff0a-4613-4a7e-8584-d41fbce79a81#overview)
2.  [Features](https://chat.qwenlm.ai/c/a85eff0a-4613-4a7e-8584-d41fbce79a81#features)
3.  [Installation](https://chat.qwenlm.ai/c/a85eff0a-4613-4a7e-8584-d41fbce79a81#installation)
4.  [Usage](https://chat.qwenlm.ai/c/a85eff0a-4613-4a7e-8584-d41fbce79a81#usage)
5.  [Contribution Guidelines](https://chat.qwenlm.ai/c/a85eff0a-4613-4a7e-8584-d41fbce79a81#contribution-guidelines)
6.  [License](https://chat.qwenlm.ai/c/a85eff0a-4613-4a7e-8584-d41fbce79a81#license)

---

## Overview

The Lumen Stellar CLI Wallet is designed to simplify interactions with the Stellar blockchain. Whether you're a developer testing smart contracts on Soroban or a user managing Stellar accounts, this CLI provides a simple and secure way to perform common tasks like generating keypairs, sending payments, and funding wallets using Friendbot.

---

## Features

-   **Generate Keypairs** : Create new Stellar public/private keypairs.
-   **Fetch Balances** : Check the balance of any Stellar account.
-   **Send XLM** : Transfer XLM between accounts securely.
-   **Keystore Management** : Save and load encrypted keystores for secure storage of private keys.
-   **Fund Wallets** : Use Stellar Testnet Friendbot or local RPC to fund accounts.
-   **Environment Integration** : Supports API keys for local RPC configurations.

## Installation

To install the Stellar Wallet CLI, follow these steps:

1. Clone the repository:

    ```sh
    git clone <repository_url>
    cd <repository_directory>
    ```

2. Build the project:

    ```sh
    cargo build --release
    ```

3. Run the CLI:
    ```sh
    ./target/release/stellar_wallet_cli
    ```

## Usage

Once the CLI is running, you can perform the following operations:

1. **Generate New Keypair**

    ```sh
    1. Generate New Keypair
    ```

2. **Fetch Balance**

    ```sh
    2. Fetch Balance
    Enter Public Key: <public_key>
    ```

3. **Send XLM**

    ```sh
    3. Send XLM
    Enter Source Secret Key: <source_secret_key>
    Enter Destination Public Key: <destination_public_key>
    Enter Amount: <amount>
    ```

4. **Save Keystore**

    ```sh
    4. Save Keystore
    Enter Password: <password>
    ```

5. **Load Keystore**

    ```sh
    5. Load Keystore
    Enter Password: <password>
    ```

6. **Fund Wallet**

    ```sh
    6. Fund Wallet
    Enter Public Key to fund: <public_key>
    ```

7. **Exit**
    ```sh
    9. Exit
    ```

## Contribution Guidelines

Contributions are welcome! If you'd like to contribute to this project, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Implement your changes.
4. Commit and push your changes.
5. Create a pull request.

Please ensure your code adheres to the project's coding standards and includes appropriate tests.

### Code Style

-   Follow Rust's idiomatic style guidelines.
-   Format your code using `cargo fmt` before submitting a PR.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
