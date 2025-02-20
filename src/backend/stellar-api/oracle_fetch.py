from requests import Request, Session
from requests.exceptions import ConnectionError, Timeout, TooManyRedirects
import json
import statistics
from datetime import datetime
import yfinance as yf
import os
import time

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


def save_file(my_list, file_name):
    # Save the list to a JSON file
    with open(file_name, "w") as file:
        json.dump(my_list, file, indent=4)  # Use `indent` for pretty formatting


def load_file(file_name):
    # Load the list from the JSON file
    with open(file_name, "r") as file:
        loaded_data = json.load(file)

    return loaded_data


def is_file_older_than(file_path, minutes=15):
    try:
        if not os.path.exists(file_path):
            print(f"File '{file_path}' does not exist.")
            return True

        current_time = time.time()
        file_mod_time = os.path.getmtime(file_path)
        time_difference = current_time - file_mod_time

        if time_difference > minutes * 60:
            return True
        else:
            return False
    except Exception as e:
        print(f"An error occurred: {e}")
        return True


def list_map_data_from_coinmarketcap(simulation_data=None):
    if simulation_data:
        data_map = simulation_data
    else:
        data_map = fetch_data_from_coinmarketcap_map()

    save_file(data_map, "cmc_top.json")

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
        tickers = ""
        # Print the sorted entries
        for entry in sorted_entries:
            print(
                f"ID: {entry['id']}, Rank: {entry['rank']}, Name: {entry['name']}, Symbol: {entry['symbol']}"
            )
            if entry["rank"] == 1:
                ids = entry["id"]
                symbols = entry["symbol"]
                tickers = f"{entry['symbol']}-USD"
            else:
                ids = f"{ids},{entry['id']}"
                symbols = f"{symbols},{entry['symbol']}"
                tickers = f"{tickers} {entry['symbol']}-USD"
            # print(entry["rank"], ids, symbols)

        # add XML
        ids = f"{ids},512"
        symbols = f"{symbols},XLM"
        tickers = f"{tickers} XLM-USD"

        return ids, symbols, tickers, sorted_entries

    except Exception as e:
        print(e)


def get_quote_data_from_coinmarketcap(simulation_data=None):
    ids, symbols, tickers, list_ranked = list_map_data_from_coinmarketcap(
        simulation_data
    )
    # ids = f"{ids},512"

    path = "v2/cryptocurrency/quotes/latest"

    parameters = {
        "id": ids,
    }

    return (
        fetch_data_from_coinmarketcap(path, parameters),
        ids,
        symbols,
        tickers,
        list_ranked,
    )


def prepare_cmc_data_for_presentation(simulation_data=None):
    if simulation_data:
        cmc_data = simulation_data
    else:
        cmc_data, ids, symbols, tickers, list_ranked = (
            get_quote_data_from_coinmarketcap()
        )

    save_file(cmc_data, "cmc_quote.json")

    # Extracting and structuring the required data
    result = []
    for _, coin_data in cmc_data["data"].items():
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
    # for item in result:
    # print(item)

    save_file(result, "cmc_data_summary.json")

    return result, ids, symbols, tickers, list_ranked


def extract_data_from_yahoo_finance_tickers(tickers, list):
    # Split the text by space
    tokens = list.split()
    result = []
    for token in tokens:
        data = tickers.tickers[token].info

        # Extracting the required fields
        extracted_data = {
            "name": data.get("name"),
            "symbol": data.get("symbol"),
            "price": data.get("regularMarketPrice"),
            "market_cap": data.get("marketCap"),
            "volume_24h": data.get("volume24Hr"),
            "volume_change_24h": data.get("regularMarketChange"),
            "percent_change_24h": data.get("regularMarketChangePercent"),
        }

        result.append(extracted_data)

    return result


def fetch_data_from_yahoo_finance(list_tickers, simulation_data=None):
    if simulation_data:
        result = simulation_data
    else:
        # Fetch data from Yahoo Finance API
        if list_tickers:
            tickers = yf.Tickers(list_tickers)
        else:
            tickers = yf.Tickers("BTC-USD ETH-USD XRP-USD")

        result = extract_data_from_yahoo_finance_tickers(tickers, list_tickers)

    save_file(result, "yf_data_summary.json")

    return result, tickers


# Combine data from  sources
def combine_data(cmc_data, yf_data):
    # Create a dictionary for quick lookup of yf_data by symbol
    yf_dict = {item["symbol"].split("-")[0]: item for item in yf_data}

    # Prepare the merged data list
    merged_data = []

    for cmc_item in cmc_data:
        symbol = cmc_item["symbol"]
        if symbol in yf_dict:
            yf_item = yf_dict[symbol]

            # Calculate averages for specific fields
            price_avg = (cmc_item["price"] + yf_item["price"]) / 2
            volume_24h_avg = (cmc_item["volume_24h"] + yf_item["volume_24h"]) / 2
            volume_change_24h_avg = (
                cmc_item["volume_change_24h"] + yf_item["volume_change_24h"]
            ) / 2
            percent_change_24h_avg = (
                cmc_item["percent_change_24h"] + yf_item["percent_change_24h"]
            ) / 2

            # Create the merged entry using cmc_item as the base
            merged_entry = {
                "cmc_rank": cmc_item["cmc_rank"],
                "id": cmc_item["id"],
                "name": cmc_item["name"],
                "symbol": cmc_item["symbol"],
                "circulating_supply": cmc_item["circulating_supply"],
                "price": price_avg,
                "market_cap": cmc_item["market_cap"],  # Use cmc_data's market_cap
                "volume_24h": volume_24h_avg,
                "volume_change_24h": volume_change_24h_avg,
                "percent_change_24h": percent_change_24h_avg,
            }
        else:
            # If no matching symbol in yf_data, use cmc_data as is
            merged_entry = cmc_item

        # Append the merged entry to the result list
        merged_data.append(merged_entry)

    save_file(merged_data, "merged_data_summary.json")

    return merged_data


def refresh_data():
    if is_file_older_than("merged_data_summary.json"):
        # ids, symbols, tickers, list_ranked = list_map_data_from_coinmarketcap()

        fresh_cmc_data, _, _, tickers, _ = prepare_cmc_data_for_presentation()

        fresh_yf_data, _ = fetch_data_from_yahoo_finance(tickers)

        merge_data = combine_data(fresh_cmc_data, fresh_yf_data)
    else:
        merge_data = load_file(("merged_data_summary.json"))

    return merge_data


if __name__ == "__main__":
    from data_simulations import cmc_quote, cmc_top_10

    # print(get_quote_data_from_coinmarketcap(cmc_top_10))
    # ids, symbols, tickers, list_ranked = list_map_data_from_coinmarketcap(cmc_top_10)
    # print(ids)
    # print(symbols)
    # print(tickers)
    # print(list_ranked)

    # prepare_cmc_data_for_presentation(cmc_quote)

    # # fetch_data_from_chainlink()
    # result, tickers = fetch_data_from_yahoo_finance(tickers)
    # # print(data.tickers["BTC-USD"].info)
    # for item in result:
    #     print(item)

    # load data
    yf_data = load_file("yf_data_summary.json")
    cmc_data = load_file("cmc_data_summary.json")

    result = combine_data(cmc_data, yf_data)
