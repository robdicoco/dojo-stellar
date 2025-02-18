from requests import Request, Session
from requests.exceptions import ConnectionError, Timeout, TooManyRedirects
import json
import statistics
from datetime import datetime

from load_config import COINMARKETCAP_API_KEY


# Off-chain data sources
def fetch_data_from_coinmarketcap(path="", parameters=""):
    if path == "" or parameters == "":
        return '{error: "Invalid path or parameters."}'

    # Replace with your CoinMarketCap API key
    url = "https://pro-api.coinmarketcap.com/" + path

    headers = {
        "Accepts": "application/json",
        "X-CMC_PRO_API_KEY": COINMARKETCAP_API_KEY,
    }

    session = Session()
    session.headers.update(headers)

    try:
        response = session.get(url, params=parameters)
        # data = json.loads(response.text)

        return response.json()

    except (ConnectionError, Timeout, TooManyRedirects) as e:
        print(e)


def fetch_data_from_coinmarketcap_map():
    path = "v1/cryptocurrency/map"

    parameters = {"limit": "10", "sort": "cmc_rank"}

    return fetch_data_from_coinmarketcap(path, parameters)


def list_map_data_from_coinmarketcap(simulation_data=None):
    if simulation_data:
        data_map = simulation_data
    else:
        data_map = fetch_data_from_coinmarketcap_map()
    try:
        # Extract unique entries based on 'id'
        unique_entries = {}
        for entry in data_map["data"]:
            if entry["id"] not in unique_entries:
                unique_entries[entry["id"]] = {
                    "id": entry["id"],
                    "rank": entry["rank"],
                    "name": entry["name"],
                    "symbol": entry["symbol"],
                }

        # Convert dictionary values to a list
        unique_entries_list = list(unique_entries.values())

        # Sort the list by 'rank'
        sorted_entries = sorted(unique_entries_list, key=lambda x: x["rank"])

        ids = ""
        symbols = ""
        # Print the sorted entries
        for entry in sorted_entries:
            print(
                f"ID: {entry['id']}, Rank: {entry['rank']}, Name: {entry['name']}, Symbol: {entry['symbol']}"
            )
            if entry["rank"] == 1:
                ids = entry["id"]
                symbols = entry["symbol"]
            else:
                ids = f"{ids},{entry['id']}"
                symbols = f"{symbols},{entry['symbol']}"
            # print(entry["rank"], ids, symbols)
        return ids, symbols, sorted_entries

    except Exception as e:
        print(e)


def get_quote_data_from_coinmarketcap(simulation_data=None):
    ids, symbols, list_ranked = list_map_data_from_coinmarketcap(simulation_data)
    ids = f"{ids},512"

    path = "v2/cryptocurrency/quotes/latest"

    parameters = {
        "id": ids,
    }

    return fetch_data_from_coinmarketcap(path, parameters)


def prepare_cmc_data_for_presentation(simulation_data=None):
    if simulation_data:
        cmc_data = simulation_data
    else:
        cmc_data = get_quote_data_from_coinmarketcap()

    # Extracting and structuring the required data
    result = []
    for coin_id, coin_data in cmc_quote["data"].items():
        quote_usd = coin_data["quote"]["USD"]
        result.append(
            {
                "cmc_rank": coin_data["cmc_rank"],
                "id": coin_data["id"],
                "name": coin_data["name"],
                "symbol": coin_data["symbol"],
                "circulating_supply": coin_data["circulating_supply"],
                "price": quote_usd["price"],
                "market_cap": quote_usd["market_cap"],
                "volume_24h": quote_usd["volume_24h"],
                "volume_change_24h": quote_usd["volume_change_24h"],
                "percent_change_24h": quote_usd["percent_change_24h"],
            }
        )

    # Sorting the result by cmc_rank
    result.sort(key=lambda x: x["cmc_rank"])

    # Printing the sorted result
    for item in result:
        print(item)


def fetch_data_from_chainlink():
    # Example: Fetch data from Chainlink API
    url = "https://api.chain.link/v2/feeds"
    response = requests.get(url)
    return response.json()


def fetch_data_from_yahoo_finance():
    # Example: Fetch data from Yahoo Finance API
    url = "https://query1.finance.yahoo.com/v7/finance/quote?symbols=BTC-USD,ETH-USD"
    response = requests.get(url)
    return response.json()


# Combine data from all sources
def combine_data(sources):
    combined_data = {}
    for source in sources:
        if source == "coinmarketcap":
            data = fetch_data_from_coinmarketcap()
            # Process data
        elif source == "chainlink":
            data = fetch_data_from_chainlink()
            # Process data
        elif source == "yahoo_finance":
            data = fetch_data_from_yahoo_finance()
            # Process data
        # Add more sources as needed
    return combined_data


# Calculate average and std deviation
def calculate_stats(data_list):
    avg = sum(data_list) / len(data_list)
    std_dev = statistics.stdev(data_list)
    return avg, std_dev


if __name__ == "__main__":
    from data_simulations import cmc_quote, cmc_top_10

    # print(get_quote_data_from_coinmarketcap(cmc_top_10))

    prepare_cmc_data_for_presentation(cmc_quote)

    # fetch_data_from_chainlink()
    # fetch_data_from_yahoo_finance()
