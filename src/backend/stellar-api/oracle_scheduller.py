from apscheduler.schedulers.background import BackgroundScheduler


def periodic_update():
    sources = ["coinmarketcap", "chainlink", "yahoo_finance"]
    combined_data = combine_data(sources)

    # Calculate averages and standard deviations
    avg_price, std_dev_price = calculate_stats(combined_data["price"])
    avg_market_cap, std_dev_market_cap = calculate_stats(combined_data["market_cap"])

    # Prepare data for Stellar
    data_to_store = {
        "rank": combined_data["rank"],
        "price": str(avg_price),
        "market_cap": str(avg_market_cap),
        "volume_24h": str(combined_data["volume_24h"]),
        "variation_24h_percent": str(combined_data["variation_24h_percent"]),
        "variation_24h_absolute": str(combined_data["variation_24h_absolute"]),
        "std_dev_price": str(std_dev_price),
        "std_dev_market_cap": str(std_dev_market_cap),
    }

    # Sign data
    signed_data = {
        key: sign_data(value, oracle_keypair.secret)
        for key, value in data_to_store.items()
    }

    # Update Stellar
    update_stellar_data_entries(signed_data)


# Schedule the task to run every 6 hours
scheduler = BackgroundScheduler()
scheduler.add_job(periodic_update, "interval", hours=6)
scheduler.start()
