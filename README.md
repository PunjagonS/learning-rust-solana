![Logo](https://jav69-public.s3.ap-southeast-1.amazonaws.com/SolaXRust.jpeg)

# 🌟 Solana Bootcamp 2024 with Anchor Framework

## Overview
Welcome to the **Solana Bootcamp 2024**! This learning project is designed to guide you through building decentralized applications using the powerful **Anchor Framework** on the Solana blockchain.

## 🎯 Learning Goals
- **Understand** the basics of the Solana blockchain.
- **Learn** how to use the Anchor Framework for building smart contracts.
- **Develop** skills in Rust and TypeScript for blockchain development.

## 🚀 Example Projects
Embark on a series of exciting projects to solidify your learning:

| Project Number | Description                       |
|----------------|-----------------------------------|
| **Project 1**  | 🌟 Building a Favorites Program   |
| **Project 2**  | 🗳️ Creating a Voting Application  |
| **Project 3**  | 🔗 Integrating Blinks and Actions |
| **Project 4**  | 🛠️ Building a CRUD Application    |
| **Project 5**  | 💰 Creating a Token               |
| **Project 6**  | 🎨 Creating a NFT                 |
| **Project 7**  | 🔄 Building a Swap Program        |
| **Project 8**  | ⏳ Creating a Token Vesting App   |
| **Project 9**  | 🎟️ Building a Token Lottery       |

---

## 🛠️ Setup Instructions

Before you can run and deploy your Solana programs, ensure you have the following tools installed:

1. **Rust and Cargo:**
   Install Rust and Cargo from [rustup.rs](https://rustup.rs/).

2. **Solana CLI:**
   Install the Solana CLI by following the instructions in the [Solana documentation](https://docs.solana.com/cli/install-solana-cli-tools).

3. **Anchor CLI:**
   Install the Anchor CLI using Cargo with the following command:
   ```bash
   cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
   ```

4. **TypeScript Types for Chai:**
   If you encounter an error indicating that TypeScript types for Chai are not found, install them using:
   ```bash
   npm install --save-dev @types/chai
   ```

## 🛠️ Running and Deploying Programs with Anchor CLI

To run and deploy your Solana programs using the Anchor CLI, follow these steps:

1. **Initialize a New Anchor Project:**
   Start a new Anchor project with the following command:
   ```bash
   anchor init solana_project
   ```

2. **Navigate to the Subproject Folder:**
   Before building or deploying, navigate to the specific subproject folder you want to work on. For example:
   ```bash
   cd project
   ```

3. **Build the Program:**
   In the subproject directory, run:
   ```bash
   anchor build
   ```
   
   When you run `anchor build`, it will automatically generate a program ID for your project. However, this ID will not be regenerated unless you run `cargo clean` followed by `anchor build` again.

4. **Start the Local Validator:**
   Start a local Solana test validator to simulate the Solana network:
   ```bash
   solana-test-validator
   ```

   **Tip for WSL Users:** If you encounter the error `Unable to connect to validator: Client error: test-ledger/admin.rpc does not exist`, try navigating to the root directory with `cd ~/` and run the command again.

5. **Deploy the Program:**
   Deploy your program to the local network:
   ```bash
   anchor deploy
   ```

These commands will help you compile, deploy, and test your Solana programs using the Anchor framework.

---

## 🔧 Useful Commands

1. **Change Network Configuration:**
   Use the following commands to change the Solana network configuration:

   - **Set to Localnet:**
     ```bash
     solana config set --url http://localhost:8899
     ```

   - **Set to Devnet:**
     ```bash
     solana config set --url https://api.devnet.solana.com
     ```

   - **Set to Testnet:**
     ```bash
     solana config set --url https://api.testnet.solana.com
     ```

   - **Set to Mainnet:**
     ```bash
     solana config set --url https://api.mainnet-beta.solana.com
     ```

   These commands configure the Solana CLI to interact with the specified network.

2. **View Wallet Address:**
   Display the public key (wallet address) from a keypair file:
   ```bash
   solana-keygen pubkey ~/.config/solana/id.json
   ```

3. **Check Balance:**
   Check the balance of your wallet:
   ```bash
   solana balance
   ```
   
4. **Request an Airdrop:**
   Request SOL tokens to be airdropped to your wallet on test networks (e.g., Devnet):
   ```bash
   solana airdrop 1
   ```

   Replace `1` with the number of SOL tokens you wish to request. This command is useful for testing purposes on networks like Devnet.

---

## 📚 Resources
- [Solana Docs](https://docs.solana.com/)
- [Anchor Framework](https://book.anchor-lang.com/)
- [Rust Programming Language](https://www.rust-lang.org/)
- [TypeScript](https://www.typescriptlang.org/)

---
