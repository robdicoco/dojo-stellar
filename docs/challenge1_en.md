[![🇧🇷 Português](https://img.shields.io/badge/Lang-PT--BR-green)](../docs/challenge1.md)
[![🇺🇸 English](https://img.shields.io/badge/Lang-EN-blue)](../docs/challenge1_en.md)

# 🚀 Dojo Stellar - Lumen League Team

<p align="center">
  <img src="../assets/images/logo_lumen.png" alt="Lumen League logo" width="450">
</p>

🌟 **Project:** Weekly challenges for learning and development within the Stellar ecosystem.

📌 **Current Challenge:** Create a **blockchain explorer** that connects to a local Stellar node.

---

## 🏆 **Challenge 1: Blockchain Explorer**

✅ **Objective:**  
Set up a **Stellar node** in the cloud and develop an explorer that connects to this node.

🔍 **The explorer must allow**:

- 🔢 **Search for a block by number**
- 🔗 **Search for a transaction by hash**
- 💰 **Check an account balance by address**

---

## ☁ **Setting Up a Stellar Node in the Cloud**

This section describes the process of installing and configuring a Stellar node in a cloud environment.

### 📖 **Step-by-Step Guide**

The tutorial covers the following aspects:

- 🔧 **Environment setup**
- 📦 **Dependency installation**
- 🔗 **Connecting to the Stellar network**
- 🔄 **Synchronizing and running the node**

📌 **Check the full guide at the link below:**

<div align="center">
  <a href="https://medium.com/@pavusa/create-your-local-stellar-node-a-step-by-step-guide-to-joining-the-stellar-network-179b80b26898" target="_blank">
    <img src="../assets/images/local_stellar_node.png" alt="Local Stellar Node Article" width="50%" style="border-radius: 10px; box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);">
  </a>
  <br>
  <a href="https://medium.com/@pavusa/create-your-local-stellar-node-a-step-by-step-guide-to-joining-the-stellar-network-179b80b26898" target="_blank">
    🔗 <strong>Access the full tutorial</strong>
  </a>
</div>

---

## 🔗 **Solution: Blockchain Explorer**

📡 The explorer must connect to the **local Stellar node** and allow interactive searches.

### 📦 **Prerequisites**

⚠ **Before starting, ensure you have installed:**  
✔ **Stellar SDK** for **Python** (`stellar-sdk`) and **JavaScript** (`stellar-sdk`)  
✔ **FastAPI** for the backend  
✔ **Vue.js** for the frontend  
✔ **Node.js** to run the Vue.js application  
✔ **Python** to run the FastAPI server  

---

## ⚙ **Backend (FastAPI)**

### 📂 **Dependencies**

To run the backend, install the following libraries:

```bash
pip install fastapi uvicorn stellar-sdk
```

- 🚀 **FastAPI**: A fast framework for building APIs in Python.  
- 🔥 **Uvicorn**: ASGI server to run FastAPI.  
- 🌐 **Stellar SDK**: Library for interacting with the Stellar blockchain.

---

## 🖥 **Frontend (Vue.js)**

### 📂 **Dependencies**

To run the frontend, install the following dependencies:

```bash
npm install axios
```

- ⚡ **Axios**: Used for making API calls to the backend.

---
