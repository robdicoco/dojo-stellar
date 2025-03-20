[![🇧🇷 Português](https://img.shields.io/badge/Lang-PT--BR-green)](./challenge2.md)
[![🇺🇸 English](https://img.shields.io/badge/Lang-EN-blue)](./challenge2-en.md)


[← Voltar para o README](../README.md)

# Dojo Stellar - Equipe Lumen League ✨

<p align="center">
  <img src="./letreiro_lumen_wallet.gif" alt="Logo da Lumen League" width="1000">
</p>

[![Status](https://img.shields.io/badge/Status-Concluído-brightgreen)](#)
![Stellar](https://img.shields.io/badge/Stellar-Blockchain-blue)
![License](https://img.shields.io/badge/License-MIT-green)

## 📜 Descrição
A **Lumen Stellar CLI Wallet** é uma ferramenta de linha de comando (CLI) desenvolvida para facilitar a interação com a blockchain **Stellar** e o serviço **Soroban RPC**. Ideal para desenvolvedores e usuários que desejam gerenciar contas Stellar, testar contratos inteligentes ou realizar transações de maneira simples e segura. Este projeto integra as funcionalidades necessárias para administração de carteiras e transações na blockchain, sendo parte do programa **Dojo Stellar – Semana 2**.

[![Wallet CLI Stellar](https://img.shields.io/badge/Wallet%20CLI-Acessar%20Agora-f5a742?style=for-the-badge&logo=stellar)](https://crates.io/crates/stellar_cli_wallet_lumen)


## 🚀 Objetivo
Fornecer uma interface de linha de comando robusta e eficiente para:
- Gerar pares de chave pública/privada.
- Consultar saldo de contas Stellar.
- Realizar transações seguras (envio de XLM).
- Gerenciar keystores para armazenamento seguro de chaves privadas.
- Financiar carteiras no Testnet usando o Friendbot.

## ⚙️ Funcionalidades
- **Geração de Chaves**: Criação de pares de chave pública/privada.  
- **Consulta de Saldo**: Verificação rápida do saldo de qualquer conta Stellar.  
- **Envio de XLM**: Transações seguras entre contas Stellar.  
- **Gestão de Keystores**: Armazenamento e recuperação de chaves privadas criptografadas.  
- **Financiamento de Contas**: Uso do Friendbot para adicionar fundos a contas de teste.  
- **Integração de Ambiente**: Suporte para configuração de APIs locais de RPC.

## 📚 Recursos e Links Úteis
- [Stellar SDK](https://developers.stellar.org/docs/tools/sdks/library)  
- [Stellar Account Viewer](https://accountviewer.stellar.org/)  
- [Soroban RPC Server](https://soroban.stellar.org/rpc)  

## 📅 Cronograma
- **Início:** 05/02/2025  
- **Entrega:** 12/02/2025  

---

## 🛠️ Tecnologias Utilizadas
- **Linguagem:** Rust  
- **Blockchain:** Stellar  
- **Serviço RPC:** Soroban  

---

## 📦 Instalação
Para instalar a **Lumen Stellar CLI Wallet**, siga os passos abaixo:

1. Clone o repositório:  
   ```bash
   git clone <repository_url>
   cd <repository_directory>
   ```

2. Compile o projeto:  
   ```bash
   cargo build --release
   ```

3. Execute o CLI:  
   ```bash
   ./target/release/stellar_wallet_cli
   ```

---

## 📚 Uso
Após executar o CLI, você poderá realizar as seguintes operações:

1. **Gerar Novo Par de Chaves**  
   ```bash
   Generate New Keypair
   ```

2. **Consultar Saldo**  
   ```bash
   Fetch Balance
   Enter Public Key: <public_key>
   ```

3. **Enviar XLM**  
   ```bash
   Send XLM
   Enter Source Secret Key: <source_secret_key>
   Enter Destination Public Key: <destination_public_key>
   Enter Amount: <amount>
   ```

4. **Salvar Keystore**  
   ```bash
   Save Keystore
   Enter Password: <password>
   ```

5. **Carregar Keystore**  
   ```bash
   Load Keystore
   Enter Password: <password>
   ```

6. **Financiar Carteira (Testnet)**  
   ```bash
   Fund Wallet
   Enter Public Key to fund: <public_key>
   ```

7. **Sair**  
   ```bash
   Exit
   ```

---

## 🤝 Contribuição
Contribuições são bem-vindas! Se deseja contribuir para este projeto, siga os passos abaixo:

1. Faça um **fork** do repositório.  
2. Crie uma **branch** para sua feature ou correção de bug.  
3. Implemente suas mudanças.  
4. **Commit** e envie as mudanças para o repositório.  
5. Crie um **pull request**.  

### Estilo de Código
- Siga as diretrizes de estilo idiomático do **Rust**.  
- Formate o código usando `cargo fmt` antes de submeter um PR.

---

## 📄 Licença
Este projeto está licenciado sob a **MIT License**. Consulte o arquivo [LICENSE](./LICENSE) para mais detalhes.

---

<p align="center">🌟 Desenvolvido durante o programa <strong>Dojo Stellar</strong> – Equipe <strong>Lumen League</strong> 🚀</p>
