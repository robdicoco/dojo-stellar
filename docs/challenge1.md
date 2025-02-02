[![🇧🇷 Português](https://img.shields.io/badge/Lang-PT--BR-green)](../docs/challenge1.md)
[![🇺🇸 English](https://img.shields.io/badge/Lang-EN-blue)](../docs/challenge1_en.md)

# 🚀 Dojo Stellar - Lumen League Team

<p align="center">
  <img src="../assets/images/logo_lumen.png" alt="Lumen League logo" width="450">
</p>

🌟 **Projeto:** Desafios semanais para o aprendizado e desenvolvimento no ecossistema Stellar.

📌 **Desafio Atual:** Criar um **explorador blockchain** que se conecte a um nó Stellar local.

---

## 🏆 **Desafio 1: Explorador Blockchain**

✅ **Objetivo:**  
Criar um nó **Stellar** na nuvem e desenvolver um explorador que se conecte a esse nó.

🔍 **O explorador deve permitir**:

- 🔢 **Buscar um bloco pelo número**
- 🔗 **Pesquisar uma transação pelo hash**
- 💰 **Consultar saldo de uma conta pelo endereço**

---

## ☁ **Configuração de um Nó Stellar na Nuvem**

Esta seção descreve o processo de instalação e configuração de um nó Stellar em um ambiente de nuvem.

### 📖 **Guia Passo a Passo**

O tutorial aborda os seguintes aspectos:

- 🔧 **Preparação do ambiente**
- 📦 **Instalação das dependências**
- 🔗 **Conexão à rede Stellar**
- 🔄 **Sincronização e execução do nó**

📌 **Consulte o guia completo no link abaixo:**

<div align="center">
  <a href="https://medium.com/@pavusa/create-your-local-stellar-node-a-step-by-step-guide-to-joining-the-stellar-network-179b80b26898" target="_blank">
    <img src="../assets/images/local_stellar_node.png" alt="Local Stellar Node Article" width="50%" style="border-radius: 10px; box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);">
  </a>
  <br>
  <a href="https://medium.com/@pavusa/create-your-local-stellar-node-a-step-by-step-guide-to-joining-the-stellar-network-179b80b26898" target="_blank">
    🔗 <strong>Acesse o tutorial completo</strong>
  </a>
</div>

---

## 🔗 **Solução: Explorador Blockchain**

📡 O explorador deve se conectar ao **nó local do Stellar** e permitir pesquisas interativas.

### 📦 **Pré-requisitos**

⚠ **Antes de iniciar, certifique-se de ter instalado:**  
✔ **Stellar SDK** para **Python** (`stellar-sdk`) e **JavaScript** (`stellar-sdk`)  
✔ **FastAPI** para o backend  
✔ **Vue.js** para o frontend  
✔ **Node.js** para rodar a aplicação Vue.js  
✔ **Python** para rodar o servidor FastAPI  

---

## ⚙ **Backend (FastAPI)**

### 📂 **Dependências**

Para rodar o backend, instale as seguintes bibliotecas:

```bash
pip install fastapi uvicorn stellar-sdk
```

- 🚀 **FastAPI**: Framework rápido para APIs em Python.  
- 🔥 **Uvicorn**: Servidor ASGI para rodar o FastAPI.  
- 🌐 **Stellar SDK**: Biblioteca para interagir com a blockchain Stellar.

---

## 🖥 **Frontend (Vue.js)**

### 📂 **Dependências**

Para rodar o frontend, instale as seguintes dependências:

```bash
npm install axios
```

- ⚡ **Axios**: Para realizar chamadas à API do backend.

---
