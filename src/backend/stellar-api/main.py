from fastapi import FastAPI, HTTPException, Request
from stellar_sdk import Server, Keypair
from pydantic import BaseModel
from slowapi import Limiter, _rate_limit_exceeded_handler
from slowapi.util import get_remote_address
from slowapi.errors import RateLimitExceeded
import re

app = FastAPI()
stellar_server = Server(horizon_url="http://localhost:8000")

# Initialize the rate limiter
limiter = Limiter(key_func=get_remote_address)  # Use the client's IP address as the key
app.state.limiter = limiter
app.add_exception_handler(RateLimitExceeded, _rate_limit_exceeded_handler)


class OracleData(BaseModel):
    rank: int
    price: float
    market_cap: float
    volume_24h: float
    variation_24h_percent: float
    variation_24h_absolute: float
    std_dev_price: float
    std_dev_market_cap: float


# Define a model for the search input
class SearchInput(BaseModel):
    query: str


@app.get("/block/{sequence}")
@limiter.limit("10/minute")
async def get_block(request: Request, sequence: int):
    # Validate the input
    if sequence <= 0:
        raise HTTPException(
            status_code=400,
            detail={
                "error": f"Invalid ledger sequence {sequence}.",
                "message": "Ledger sequence must be a positive integer.",
            },
        )

    try:
        # Fetch the requested ledger
        ledger = stellar_server.ledgers().ledger(sequence).call()
        return ledger
    except Exception as e:
        # Handle the case where the ledger is not found
        error_message = str(e)

        if (
            "prior to the recorded history known" in error_message
            or "invalid in some way" in error_message
        ):
            try:
                # Fetch the latest ledger
                latest_ledger = (
                    stellar_server.ledgers().order(desc=True).limit(1).call()
                )
                latest_sequence = latest_ledger["_embedded"]["records"][0]["sequence"]

                # Fetch the earliest ledger
                earliest_ledger = (
                    stellar_server.ledgers().order(desc=False).limit(1).call()
                )
                earliest_sequence = earliest_ledger["_embedded"]["records"][0][
                    "sequence"
                ]

                # Check if the requested sequence is out of range
                if int(sequence) < int(earliest_sequence):
                    raise HTTPException(
                        status_code=404,
                        detail={
                            "error": f"Ledger with sequence {sequence} not found.",
                            "valid_range": f"Valid ledger sequence numbers are between {earliest_sequence} and {latest_sequence}.",
                        },
                    )
                elif int(sequence) > int(latest_sequence):
                    raise HTTPException(
                        status_code=404,
                        detail={
                            "error": f"Ledger with sequence {sequence} not found.",
                            "valid_range": f"Valid ledger sequence numbers are between {earliest_sequence} and {latest_sequence}.",
                        },
                    )
                else:
                    # This should never happen unless there's a gap in the ledger history
                    raise HTTPException(
                        status_code=404,
                        detail={
                            "error": f"Ledger with sequence {sequence} not found.",
                            "valid_range": f"Valid ledger sequence numbers are between {earliest_sequence} and {latest_sequence}.",
                        },
                    )
            except Exception as inner_e:
                error_message2 = str(inner_e)
                if "Ledger with sequence" in error_message2:
                    raise inner_e

                # Log the inner exception for debugging
                print(f"Inner exception while fetching ledger range: {inner_e}")
                raise HTTPException(
                    status_code=500,
                    detail=f"An unexpected error occurred while determining the valid ledger range",
                )
        else:
            # Handle other unexpected errors
            raise HTTPException(
                status_code=500, detail=f"An unexpected error occurred: {str(e)}"
            )


@app.get("/transaction/{hash}")
@limiter.limit("10/minute")
async def get_transaction(request: Request, hash: str):
    try:
        transaction = stellar_server.transactions().transaction(hash).call()
        return transaction
    except Exception as e:
        raise HTTPException(status_code=404, detail=str(e))


@app.get("/balance/{address}")
@limiter.limit("10/minute")
async def get_balance(request: Request, address: str):
    try:
        account = stellar_server.accounts().account_id(address).call()
        balances = account["balances"]
        return balances
    except Exception as e:
        raise HTTPException(status_code=404, detail=str(e))


@app.get("/account/{address}")
@limiter.limit("10/minute")
async def get_account_info(request: Request, address: str):
    try:
        account = stellar_server.accounts().account_id(address).call()
        return account
    except Exception as e:
        raise HTTPException(
            status_code=400, detail=f"Error fetching account info: {str(e)}"
        )


@app.post("/search")
@limiter.limit("20/minute")
async def search(request: Request, query: SearchInput):
    # Extract the query string
    search_query = query.query.strip()

    try:
        if re.match(r"^[0-9]+$", search_query):
            # If the query is a number, assume it's a block/ledger number
            ledger_info = await get_block(request, search_query)
            return {"result": "ledger_info", "data": ledger_info}

        elif re.match(r"^[a-fA-F0-9]{64}$", search_query):
            # If the query is a hash, assume it's a transaction hash
            transaction_info = await get_transaction(request, search_query)
            return {"result": "transaction_info", "data": transaction_info}

        else:
            # Assume it's an address
            account_info = await get_balance(request, search_query)
            return {"result": "account_info", "data": account_info}

    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))


# Get Blockchain Info
@app.get("/block_info")
@limiter.limit("10/minute")
async def get_blockchain_info(
    request: Request,
):
    try:
        latest_ledger = (
            stellar_server.ledgers()
            .order(desc=True)
            .limit(1)
            .call()["_embedded"]["records"][0]
        )
        return {
            "latest_ledger_sequence": latest_ledger["sequence"],
            "latest_ledger_hash": latest_ledger["hash"],
            "ledger_count": latest_ledger["sequence"],  # Total ledgers up to this point
            "protocol_version": latest_ledger["protocol_version"],
        }
    except Exception as e:
        raise HTTPException(
            status_code=400, detail=f"Error fetching blockchain info: {str(e)}"
        )


# List Latest 5 Transactions
@app.get("/latest_transactions")
@limiter.limit("10/minute")
async def get_latest_transactions(request: Request):
    try:
        transactions = (
            stellar_server.transactions()
            .order(desc=True)
            .limit(5)
            .call()["_embedded"]["records"]
        )
        return transactions
    except Exception as e:
        raise HTTPException(
            status_code=400, detail=f"Error fetching latest transactions: {str(e)}"
        )


# List Latest 5 Ledgers
@app.get("/latest_ledgers")
@limiter.limit("10/minute")
async def get_latest_ledgers(request: Request):
    try:
        ledgers = (
            stellar_server.ledgers()
            .order(desc=True)
            .limit(5)
            .call()["_embedded"]["records"]
        )
        return ledgers
    except Exception as e:
        raise HTTPException(
            status_code=400, detail=f"Error fetching latest ledgers: {str(e)}"
        )


# In-memory storage for latest and historical data
latest_data = None
historical_data = []


@app.post("/update_oracle_data/")
async def update_oracle_data(data: OracleData):
    global latest_data
    latest_data = data.dict()
    historical_data.append(latest_data)
    if len(historical_data) > 10:
        historical_data.pop(0)
    return {"status": "success"}


@app.get("/latest_data/")
async def get_latest_data():
    return latest_data


@app.get("/historical_data/")
async def get_historical_data():
    return historical_data[-10:]


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=7000)
