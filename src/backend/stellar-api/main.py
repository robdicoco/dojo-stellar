from fastapi import FastAPI, HTTPException
from stellar_sdk import Server, Keypair
from pydantic import BaseModel
import re

app = FastAPI()
stellar_server = Server(horizon_url="http://localhost")


# Define a model for the search input
class SearchInput(BaseModel):
    query: str


@app.get("/block/{sequence}")
async def get_block(sequence: int):
    try:
        ledger = stellar_server.ledgers().ledger(sequence).call()
        return ledger
    except Exception as e:
        raise HTTPException(status_code=404, detail=str(e))


@app.get("/transaction/{hash}")
async def get_transaction(hash: str):
    try:
        transaction = stellar_server.transactions().transaction(hash).call()
        return transaction
    except Exception as e:
        raise HTTPException(status_code=404, detail=str(e))


@app.get("/balance/{address}")
async def get_balance(address: str):
    try:
        account = stellar_server.accounts().account_id(address).call()
        balances = account["balances"]
        return balances
    except Exception as e:
        raise HTTPException(status_code=404, detail=str(e))


@app.post("/search")
async def search(query: SearchInput):
    # Extract the query string
    search_query = query.query.strip()

    try:
        if re.match(r"^[0-9]+$", search_query):
            # If the query is a number, assume it's a block/ledger number
            ledger_info = await get_ledger_info(search_query)
            return {"result": "ledger_info", "data": ledger_info}

        elif re.match(r"^[a-fA-F0-9]{64}$", search_query):
            # If the query is a hash, assume it's a transaction hash
            transaction_info = await get_transaction_info(search_query)
            return {"result": "transaction_info", "data": transaction_info}

        else:
            # Assume it's an address
            account_info = await get_account_info(search_query)
            return {"result": "account_info", "data": account_info}

    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))


async def get_ledger_info(ledger_number):
    try:
        ledger = stellar_server.ledgers().ledger(ledger_number).call()
        return ledger
    except Exception as e:
        raise HTTPException(
            status_code=400, detail=f"Error fetching ledger info: {str(e)}"
        )


async def get_transaction_info(transaction_hash):
    try:
        transaction = stellar_server.transactions().transaction(transaction_hash).call()
        return transaction
    except Exception as e:
        raise HTTPException(
            status_code=400, detail=f"Error fetching transaction info: {str(e)}"
        )


async def get_account_info(account_address):
    try:
        account = stellar_server.accounts().account_id(account_address).call()
        return account
    except Exception as e:
        raise HTTPException(
            status_code=400, detail=f"Error fetching account info: {str(e)}"
        )


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=7000)
