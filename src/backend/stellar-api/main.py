from fastapi import FastAPI, HTTPException
from stellar_sdk import Server, Keypair

app = FastAPI()
server = Server(horizon_url="http://localhost")

@app.get("/block/{sequence}")
async def get_block(sequence: int):
    try:
        ledger = server.ledgers().ledger(sequence).call()
        return ledger
    except Exception as e:
        raise HTTPException(status_code=404, detail=str(e))

@app.get("/transaction/{hash}")
async def get_transaction(hash: str):
    try:
        transaction = server.transactions().transaction(hash).call()
        return transaction
    except Exception as e:
        raise HTTPException(status_code=404, detail=str(e))

@app.get("/balance/{address}")
async def get_balance(address: str):
    try:
        account = server.accounts().account_id(address).call()
        balances = account['balances']
        return balances
    except Exception as e:
        raise HTTPException(status_code=404, detail=str(e))

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=7000)