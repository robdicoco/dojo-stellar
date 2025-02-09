## End Point Validationss

### 1. **Get Block by Sequence**

#### Valid ledger sequence

    curl -X GET "https://lumen.758206.xyz:7001/block/8" -k

#### Invalid ledger sequence (out of range)

    curl -X GET "https://lumen.758206.xyz:7001/block/999999999999" -k

---

### 2. **Get Transaction by Hash**

#### Valid transaction hash

    curl -X GET "https://lumen.758206.xyz:7001/transaction/d0a1e6b3c4f5d6e7f8g9h0i1j2k3l4m5n6o7p8q9r0s1t2u3v4w5x6y7z" -k

#### Invalid transaction hash

    curl -X GET "https://lumen.758206.xyz:7001/transaction/invalidhash123" -k

---

### 3. **Get Balance by Address**

#### Valid Stellar address

    curl -X GET "https://lumen.758206.xyz:7001/balance/GABC1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcd" -k

#### Invalid Stellar address

    curl -X GET "https://lumen.758206.xyz:7001/balance/invalidaddress123" -k

---

### 4. **Get Account Info by Address**

#### Valid Stellar address

    curl -X GET "https://lumen.758206.xyz:7001/account/GABC1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcd" -k

#### Invalid Stellar address

    curl -X GET "https://lumen.758206.xyz:7001/account/invalidaddress123" -k

---

### 5. **Search Endpoint**

#### Search with a valid ledger sequence

    curl -X POST "https://lumen.758206.xyz:7001/search" -H "Content-Type: application/json" -d '{"query": "1"}' -k

#### Search with a valid transaction hash

    curl -X POST "https://lumen.758206.xyz:7001/search" -H "Content-Type: application/json" -d '{"query": "d0a1e6b3c4f5d6e7f8g9h0i1j2k3l4m5n6o7p8q9r0s1t2u3v4w5x6y7z"}' -k

#### Search with a valid Stellar address

    curl -X POST "https://lumen.758206.xyz:7001/search" -H "Content-Type: application/json" -d '{"query": "GABC1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcd"}' -k

#### Search with an invalid query

    curl -X POST "https://lumen.758206.xyz:7001/search" -H "Content-Type: application/json" -d '{"query": "invalidquery123"}' -k

---

### 6. **Get Blockchain Info**

    curl -X GET "https://lumen.758206.xyz:7001/block_info" -k

---

### 7. **Get Latest Transactions**

    curl -X GET "https://lumen.758206.xyz:7001/latest_transactions" -k

---

### 8. **Get Latest Ledgers**

    curl -X GET "https://lumen.758206.xyz:7001/latest_ledgers" -k

---

### 9. **Rate Limiting Test**

You can test rate limiting by repeatedly hitting an endpoint like `/block/{sequence}`:

#### Make 10 requests within the allowed limit

    for i in {1..10}; do curl -X GET "https://lumen.758206.xyz:7001/block/10" -k; done

#### Make one more request to exceed the limit

    curl -X GET "https://lumen.758206.xyz:7001/block/1" -k
