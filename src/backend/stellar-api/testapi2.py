import pytest
import httpx
import re

# Base URL of the API
BASE_URL = "https://lumen.758206.xyz:7001"


# Helper function to make requests
async def make_request(method, endpoint, **kwargs):
    async with httpx.AsyncClient(
        verify=False
    ) as client:  # Disable SSL verification for self-signed certs
        response = await client.request(method, f"{BASE_URL}{endpoint}", **kwargs)
        return response


# Test /block/{sequence}
@pytest.mark.asyncio
async def test_get_block():
    # Test with a valid ledger sequence
    response = await make_request("GET", "/block/10")
    assert response.status_code == 200
    assert "sequence" in response.json()

    # Test with an invalid ledger sequence
    response = await make_request("GET", "/block/999999999999")
    assert response.status_code == 404
    assert "detail" in response.json()
    assert "valid_range" in response.json()["detail"]

    # Test with an invalid ledger sequence
    response = await make_request("GET", "/block/-1")
    assert response.status_code == 400
    assert "detail" in response.json()
    assert "error" in response.json()["detail"]

    # Test with an invalid ledger sequence
    response = await make_request("GET", "/block/2")
    assert response.status_code == 404
    assert "detail" in response.json()
    assert "valid_range" in response.json()["detail"]


# Test /transaction/{hash}
@pytest.mark.asyncio
async def test_get_transaction():
    # Test with a valid transaction hash
    valid_hash = "4395cef97b0553e2d58595c9eb6f0392726d31883b9eb10da5e3feb1a51e61df"
    response = await make_request("GET", f"/transaction/{valid_hash}")
    if response.status_code == 200:
        assert "hash" in response.json()
    else:
        assert response.status_code == 404
        assert "detail" in response.json()

    # Test with an invalid transaction hash
    invalid_hash = "invalidhash123"
    response = await make_request("GET", f"/transaction/{invalid_hash}")
    assert response.status_code == 404
    assert "detail" in response.json()


# Test /balance/{address}
@pytest.mark.asyncio
async def test_get_balance():
    # Test with a valid Stellar address
    valid_address = "GCKFLO7RHFB2HXMEN6EMZPT6WYCIOL3WJPDJWGIKBQA2KAVS6VS4WY4E"
    response = await make_request("GET", f"/balance/{valid_address}")
    if response.status_code == 200:
        assert isinstance(response.json(), list)
    else:
        assert response.status_code == 404
        assert "detail" in response.json()

    # Test with an invalid Stellar address
    invalid_address = "invalidaddress123"
    response = await make_request("GET", f"/balance/{invalid_address}")
    assert response.status_code == 404
    assert "detail" in response.json()


# Test /account/{address}
@pytest.mark.asyncio
async def test_get_account_info():
    # Test with a valid Stellar address
    valid_address = "GCKFLO7RHFB2HXMEN6EMZPT6WYCIOL3WJPDJWGIKBQA2KAVS6VS4WY4E"
    response = await make_request("GET", f"/account/{valid_address}")
    if response.status_code == 200:
        assert "id" in response.json()
    else:
        assert response.status_code == 400
        assert "detail" in response.json()

    # Test with an invalid Stellar address
    invalid_address = "invalidaddress123"
    response = await make_request("GET", f"/account/{invalid_address}")
    assert response.status_code == 400
    assert "detail" in response.json()


# Test /search
@pytest.mark.asyncio
async def test_search():
    # Test with a valid ledger sequence
    response = await make_request("POST", "/search", json={"query": "13"})
    if response.status_code == 200:
        assert "result" in response.json()
        assert response.json()["result"] == "ledger_info"
    else:
        assert response.status_code == 400
        assert "detail" in response.json()

    # Test with a valid transaction hash
    valid_hash = "4395cef97b0553e2d58595c9eb6f0392726d31883b9eb10da5e3feb1a51e61df"
    response = await make_request("POST", "/search", json={"query": valid_hash})
    if response.status_code == 200:
        assert "result" in response.json()
        assert response.json()["result"] == "transaction_info"
    else:
        assert response.status_code == 400
        assert "detail" in response.json()

    # Test with a valid Stellar address
    valid_address = "GCKFLO7RHFB2HXMEN6EMZPT6WYCIOL3WJPDJWGIKBQA2KAVS6VS4WY4E"
    response = await make_request("POST", "/search", json={"query": valid_address})
    if response.status_code == 200:
        assert "result" in response.json()
        assert response.json()["result"] == "account_info"
    else:
        assert response.status_code == 400
        assert "detail" in response.json()

    # Test with an invalid query
    invalid_query = "invalidquery123"
    response = await make_request("POST", "/search", json={"query": invalid_query})
    assert response.status_code == 400
    assert "detail" in response.json()


# Test /block_info
@pytest.mark.asyncio
async def test_get_blockchain_info():
    response = await make_request("GET", "/block_info")
    assert response.status_code == 200
    assert "latest_ledger_sequence" in response.json()
    assert "latest_ledger_hash" in response.json()
    assert "ledger_count" in response.json()
    assert "protocol_version" in response.json()


# Test /latest_transactions
@pytest.mark.asyncio
async def test_get_latest_transactions():
    response = await make_request("GET", "/latest_transactions")
    assert response.status_code == 200
    assert isinstance(response.json(), list)
    assert len(response.json()) <= 5


# Test /latest_ledgers
@pytest.mark.asyncio
async def test_get_latest_ledgers():
    response = await make_request("GET", "/latest_ledgers")
    assert response.status_code == 200
    assert isinstance(response.json(), list)
    assert len(response.json()) <= 5


# Test rate limiting
# @pytest.mark.asyncio
# async def test_rate_limiting():
#     endpoint = "/block/1"
#     for _ in range(10):  # Make 10 requests within the limit
#         response = await make_request("GET", endpoint)
#         assert response.status_code == 200

#     # Make one more request to exceed the limit
#     response = await make_request("GET", endpoint)
#     assert response.status_code == 429  # Too Many Requests
#     assert "detail" in response.json()
