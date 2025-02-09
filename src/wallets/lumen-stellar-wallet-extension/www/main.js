import init, {
    create_keypair,
    encrypt_data,
    decrypt_data,
    fetch_balance,
} from "./pkg/stellar_wallet_extension.js";

async function run() {
    await init();

    // Example: Create a new keypair
    document.getElementById("createWallet").addEventListener("click", () => {
        const result = create_keypair();
        const { publicKey, secretSeed } = JSON.parse(result);
        alert(`Public Key: ${publicKey}\nSecret Seed: ${secretSeed}`);
    });

    // Example: Fetch balance
    document
        .getElementById("fetchBalance")
        .addEventListener("click", async () => {
            const publicKey = prompt("Enter your public key:");
            const rpcUrl = "https://horizon-testnet.stellar.org";
            const balance = await fetch_balance(publicKey, rpcUrl);
            alert(`Balance: ${balance} XLM`);
        });
}

run();
