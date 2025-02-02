# Dojo Stellar - Lumen League team

<img src="logo.png" alt="Lumen League logo" width="200">

## Challenge 1

-   Create Stellar node on any Cloud

-   Create an Explorer that connects to the local node of stellar, it
    must:
    -   Search for a block by number
    -   Search for a transaction by hash
    -   Search for the balance by address

## Solution: Stellar node on Cloud

<a href="https://medium.com/@pavusa/create-your-local-stellar-node-a-step-by-step-guide-to-joining-the-stellar-network-179b80b26898" target="_blank">Create Your Local Stellar Node: A Step-by-Step Guide to Joining the Stellar Network!
<br />
<img src="Local Stellar Node.png" alt="Local Stellar Node Article" width="300"></a>

## Solution: Explorer that connects to the local node of stellar

### Prerequisites

1.  **Stellar SDK**: You'll need the Stellar SDK for Python (`stellar-sdk`) for the backend and JavaScript (`stellar-sdk`) for the frontend.
2.  **FastAPI**: For the backend API.
3.  **Vue.js**: For the frontend.
4.  **Node.js**: For running the Vue.js application.
5.  **Python**: For running the FastAPI server.

### Backend (FastAPI)

#### Dependencies

-   fastapi
-   uvicorn
-   stellar-sdk

### Frontend (Vue.js)

#### Dependencies

-   axios
