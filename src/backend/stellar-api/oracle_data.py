from stellar_sdk import (
    Server,
    Keypair,
    TransactionBuilder,
    Network,
    TransactionEnvelope,
    xdr,
    ManageData,
)
import time
import hashlib
import hmac
import base64


from load_config import STELLAR_URL, ORACLE_ACCOUNT_SECRET

# Stellar Testnet server
server = Server(STELLAR_URL)

# Oracle Stellar Account
oracle_keypair = Keypair.from_secret(ORACLE_ACCOUNT_SECRET)
oracle_account = server.load_account(oracle_keypair.public_key)


# Function to update Stellar Data Entries
def update_stellar_data_entries(data):
    """
    Updates the Stellar account's data entries with the signed data.
    """
    try:
        # Fetch the current sequence number of the Stellar account
        transaction = TransactionBuilder(
            source_account=oracle_account,
            network_passphrase=Network.TESTNET_NETWORK_PASSPHRASE,
            base_fee=100,
        ).add_text_memo("Oracle Update")

        transaction.set_timeout(30)

        # Add TimeBounds to the transaction
        current_time = int(time.time())
        transaction.add_time_bounds(
            min_time=current_time, max_time=current_time + 300
        )  # 5-minute window

        for key, value in data.items():
            transaction.append_manage_data_op(data_name=key, data_value=value)
            transaction.append_manage_data_op(
                data_name=f"{key}_s", data_value=sign_data(value, ORACLE_ACCOUNT_SECRET)
            )

        # print(transaction)

        transaction = transaction.build()
        # print(transaction)
        transaction.sign(oracle_keypair)
        response = server.submit_transaction(transaction)

        print("Transaction successful:", response)
        return response
    except Exception as e:
        print("Error updating Stellar data entries:", str(e))
        return str(e)


def sign_data(data, secret_key):
    signature = hmac.new(secret_key.encode(), data.encode(), hashlib.sha256).hexdigest()
    return signature


def decode_transaction_envelope(envelope_xdr, secret_key):
    """Decode the transaction envelope and extract ManageData operations."""
    # Decode the envelope_xdr
    try:
        # Parse the transaction envelope from XDR
        transaction_envelope = TransactionEnvelope.from_xdr(
            envelope_xdr, network_passphrase=Network.TESTNET_NETWORK_PASSPHRASE
        )
        transaction = transaction_envelope.transaction

        # print(transaction)

        # Extract manage_data operations
        manage_data_operations = [
            op for op in transaction.operations if isinstance(op, ManageData)
        ]

        if not manage_data_operations:
            raise ValueError("No manage_data operations found in the transaction")

        # print(manage_data_operations)

        # Decode and return the data
        decoded_data = {}
        # Filter out keys that end with "_s"
        # filtered_keys = [k for k in account['data'].keys() if not k.endswith("_s")]

        for op in manage_data_operations:
            key = op.data_name  # Decode the key from bytes
            if key.endswith("_s"):
                continue

            key_value = op.data_value.decode("utf-8")  # Decode the value from bytes

            # Decode the signed value
            try:
                decoded_value = key_value
                decoded_data[key] = decoded_value
            except ValueError as e:
                decoded_data[key] = f"Error decoding: {e}"

        return decoded_data

    except Exception as e:
        raise ValueError(f"Failed to decode XDR: {e}")


# Function to retrieve the latest transaction and extract manage_data operations
def get_latest_transaction_data(account_id, secret_key):
    try:
        account = server.accounts().account_id(account_id).call()
        # print("Account exists:", account)
    except Exception as e:
        print("Account does not exist or error:", e)
        return e

    # Fetch the latest transactions for the given account
    transactions = (
        server.transactions().for_account(account_id).order(desc=True).limit(1).call()
    )

    if not transactions["_embedded"]["records"]:
        raise ValueError("No transactions found for the given account")

    # Get the latest transaction
    latest_transaction = transactions["_embedded"]["records"][0]
    transaction_hash = latest_transaction["hash"]

    # print(latest_transaction["envelope_xdr"])

    # Fetch the details of the latest transaction
    try:
        operation_details = decode_transaction_envelope(
            latest_transaction["envelope_xdr"], secret_key
        )
    except Exception as e:
        raise ValueError(f"Failed to fetch transaction details: {e}")

    # print(operation_details)

    return operation_details


def get_oracle_latest_stored_data():
    try:
        # Retrieve and decode the latest transaction data
        decoded_data = get_latest_transaction_data(
            oracle_keypair.public_key, oracle_keypair
        )
        # print("Decoded Data:")
        # for key, value in decoded_data.items():
        #     print(f"{key}: {value}")

        return decoded_data
    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    get_oracle_latest_stored_data()
