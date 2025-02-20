from stellar_sdk import Server, Keypair, TransactionBuilder, Network

import hashlib
import hmac


from load_config import STELLAR_URL, ORACLE_ACCOUNT_SECRET

# Stellar Testnet server
server = Server(STELLAR_URL)

# Oracle Stellar Account
oracle_keypair = Keypair.from_secret(ORACLE_ACCOUNT_SECRET)
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


def update_stellar_data_entries2(signed_data):
    """
    Updates the Stellar account's data entries with the signed data.
    """
    try:
        # Fetch the current sequence number of the Stellar account
        oracle_account = server.load_account(oracle_keypair.public_key)

        # Build the transaction to update data entries
        transaction = (
            TransactionBuilder(
                source_account=oracle_account,
                network_passphrase=Network.TESTNET_NETWORK_PASSPHRASE,  # Use TESTNET for development
                base_fee=100,
            )
            .add_text_memo("Oracle Update")
            .set_timeout(30)  # Set timeout for the transaction
        )

        # Add operations to update each data entry
        for key, value in signed_data.items():
            transaction.append_manage_data_op(key, value)

        # Sign and submit the transaction
        signed_transaction = transaction.build().sign(oracle_keypair)
        response = server.submit_transaction(signed_transaction)
        print("Transaction successful:", response)
    except Exception as e:
        print("Error updating Stellar data entries:", str(e))


def sign_data(data, secret_key):
    signature = hmac.new(secret_key.encode(), data.encode(), hashlib.sha256).hexdigest()
    return signature


def sign_data2(value, secret_key):
    """
    Signs the given value using the Stellar account's secret key.
    This ensures data integrity and authenticity.
    """
    signer_keypair = Keypair.from_secret(secret_key)
    signature = signer_keypair.sign(value.encode()).hex()
    return signature
