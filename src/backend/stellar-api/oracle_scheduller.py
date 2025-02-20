from apscheduler.schedulers.background import BackgroundScheduler
from stellar_sdk import Server, Keypair, TransactionBuilder, Network


from oracle_data import update_stellar_data_entries, sign_data2
from oracle_fetch import refresh_data

from load_config import ORACLE_ACCOUNT_SECRET


# Oracle Stellar Account
oracle_keypair = Keypair.from_secret(ORACLE_ACCOUNT_SECRET)


def periodic_update():
    current_data = refresh_data()

    prepare_and_store_data(current_data)


def prepare_and_store_data(fresh_data):
    """
    Prepares the fresh_data for storage in the Stellar account and updates it.
    """
    for entry in fresh_data:
        # Extract relevant fields
        symbol = entry["symbol"]
        data_to_store = {
            f"{symbol}_rank": str(entry["cmc_rank"]),
            f"{symbol}_price": str(entry["price"]),
            f"{symbol}_market_cap": str(entry["market_cap"]),
            f"{symbol}_volume_24h": str(entry["volume_24h"]),
            f"{symbol}_volume_change_24h": str(entry["volume_change_24h"]),
            f"{symbol}_percent_change_24h": str(entry["percent_change_24h"]),
        }

        # Sign each piece of data
        signed_data = {
            key: sign_data2(value, ORACLE_ACCOUNT_SECRET)
            for key, value in data_to_store.items()
        }

        # Update Stellar with the signed data
        update_stellar_data_entries(signed_data)


def start_scheduler(scheduler):
    # Schedule the task to run every 12 hours
    scheduler.add_job(periodic_update, "interval", hours=12)
    scheduler.start()

    # Keep the script running
    try:
        while True:

            pass
    except (KeyboardInterrupt, SystemExit):
        scheduler.shutdown()


if __name__ == "__main__":
    periodic_update()

    # scheduler = BackgroundScheduler()

    # scheduler.shutdown()
