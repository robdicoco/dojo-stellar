import json

# Default values
DEFAULT_STELLAR_URL = "https://horizon-testnet.stellar.org"
DEFAULT_ORACLE_ACCOUNT_SECRET = "YOUR_ORACLE_ACCOUNT_SECRET"
DEFAULT_ORACLE_ACCOUNT_PUBLIC = (
    "GDA2US4DI3G2EC63KSLLJFRJ7QOWNZ4MKCTAHQXSYZN34ZYD4DC7SR2I"
)
DEFAULT_NETWORK_PASSPHRASE = "Test SDF Network ; September 2015"
DEFAULT_COINMARKETCAP_API_KEY = "38334288-74ce-45c3-8a2f-5b901f962f2e"
DEFAULT_CHAINLINK_API_URL = "https://api.chain.link/data-feeds"
DEFAULT_YAHOO_API_URL = "https://yahoo-finance-api.com"

# Load the JSON file
try:
    with open("env.json", "r") as file:
        config = json.load(file)
except FileNotFoundError as e:
    print("Error: env.json file not found. Using default values.\n" + e)
    config = {}

# Assign variables from the JSON file, using defaults if keys are missing
STELLAR_URL = config.get("stellar_url", DEFAULT_STELLAR_URL)
ORACLE_ACCOUNT_SECRET = config.get(
    "oracle_account_secret", DEFAULT_ORACLE_ACCOUNT_SECRET
)
ORACLE_ACCOUNT_PUBLIC = config.get(
    "oracle_account_public", DEFAULT_ORACLE_ACCOUNT_PUBLIC
)
NETWORK_PASSPHRASE = config.get("network_passphrase", DEFAULT_NETWORK_PASSPHRASE)
COINMARKETCAP_API_KEY = config.get(
    "coinmarketcap_api_key", DEFAULT_COINMARKETCAP_API_KEY
)
CHAINLINK_API_URL = config.get("chainlink_api_url", DEFAULT_CHAINLINK_API_URL)
YAHOO_API_URL = config.get("yahoo_api_url", DEFAULT_YAHOO_API_URL)


if __name__ == "__main__":
    # Print variables to verify
    print("STELLAR_TESTNET_URL:", STELLAR_URL)
    print("ORACLE_ACCOUNT_SECRET:", ORACLE_ACCOUNT_SECRET)
    print("ORACLE_ACCOUNT_PUBLIC:", ORACLE_ACCOUNT_PUBLIC)
    print("NETWORK_PASSPHRASE:", NETWORK_PASSPHRASE)
    print("COINMARKETCAP_API_KEY:", COINMARKETCAP_API_KEY)
    print("CHAINLINK_API_URL:", CHAINLINK_API_URL)
    print("YAHOO_API_URL:", YAHOO_API_URL)
