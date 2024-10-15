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

1. **Navigate to the Subproject Folder:**
   Before building or deploying, navigate to the specific subproject folder you want to work on. For example:
   ```bash
   cd project
   ```

2. **Build the Program:**
   In the subproject directory, run:
   ```bash
   anchor build
   ```

3. **Start the Local Validator:**
   Start a local Solana test validator to simulate the Solana network:
   ```bash
   solana-test-validator
   ```

   **Tip for WSL Users:** If you encounter the error `Unable to connect to validator: Client error: test-ledger/admin.rpc does not exist`, try navigating to the root directory with `cd ~/` and run the command again.

4. **Deploy the Program:**
   Deploy your program to the local network:
   ```bash
   anchor deploy
   ```

5. **Run Tests:**
   If you have written tests for your program, execute them using:
   ```bash
   anchor test
   ```

These commands will help you compile, deploy, and test your Solana programs using the Anchor framework.

---

## 📚 Resources
- [Solana Docs](https://docs.solana.com/)
- [Anchor Framework](https://book.anchor-lang.com/)
- [Rust Programming Language](https://www.rust-lang.org/)
- [TypeScript](https://www.typescriptlang.org/)

---
