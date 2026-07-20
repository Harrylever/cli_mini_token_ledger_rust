# Mini Token Ledger

A simple command-line in-memory token ledger built with Rust.

## Features

- **Create Wallet**: Create a new wallet address (`create <name>`)
- **Delete Wallet**: Remove an existing empty wallet (`del <name>`)
- **Mint Tokens**: Mint tokens into a specified wallet (`mint <to> <amount>`)
- **Burn Tokens**: Burn tokens from a specified wallet (`burn <from> <amount>`)
- **Transfer Tokens**: Transfer tokens between two wallets (`transfer <from> <to> <amount>`)
- **Check Balance**: View the balance of a specific wallet (`balance <name>`)
- **Show Wallets**: Display all registered wallets and their current balances (`show`)

## Prerequisites

- [Rust](https://www.rust-lang.org/) (2024 edition supported)

## Usage

Run the application using Cargo:

```bash
cargo run
```

### Example Interactive Commands

```text
create alice
create bob
mint alice 100
transfer alice bob 30
balance bob
show
exit
```
