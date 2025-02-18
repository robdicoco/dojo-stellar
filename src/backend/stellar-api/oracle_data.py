from stellar_sdk import Server, Keypair, TransactionBuilder, Network

import hashlib
import hmac

# Stellar Testnet server
server = Server("https://horizon-testnet.stellar.org")

# Oracle Stellar Account
oracle_keypair = Keypair.from_secret("YOUR_SECRET_KEY")
oracle_account = server.load_account(oracle_keypair.public_key)


# Function to update Stellar Data Entries
def update_stellar_data_entries(data):
    transaction = TransactionBuilder(
        source_account=oracle_account,
        network_passphrase=Network.TESTNET_NETWORK_PASSPHRASE,
        base_fee=100,
    ).add_text_memo("Oracle Update")

    for key, value in data.items():
        transaction.append_manage_data_op(key, value)

    transaction = transaction.build()
    transaction.sign(oracle_keypair)
    response = server.submit_transaction(transaction)
    return response


def sign_data(data, secret_key):
    signature = hmac.new(secret_key.encode(), data.encode(), hashlib.sha256).hexdigest()
    return signature
