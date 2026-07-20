use std::{
    collections::HashMap,
    io::{self, Write},
};

#[derive(Debug)]
enum TransferError {
    InsufficientFunds,
    InvalidTransfer,
    AccountNotFound,
    InvalidAmount,
}

#[derive(Debug)]
enum CreateWalletError {
    AccountAlreadyExists,
    NameTooShort,
    NameTooLong,
}

#[derive(Debug)]
enum DeleteWalletError {
    AccountNotFound,
    NotEmpty,
}

fn transfer(
    wallet_addresses: &mut HashMap<String, u64>,
    from: String,
    to: String,
    amount: u64,
) -> Result<(), TransferError> {
    if !wallet_addresses.contains_key(&from) {
        println!("Sender not found!");
        return Err(TransferError::AccountNotFound);
    }

    if !wallet_addresses.contains_key(&to) {
        println!("Receiver not found!");
        return Err(TransferError::AccountNotFound);
    }

    if from == to {
        println!("Can't transfer to the same account!");
        return Err(TransferError::InvalidTransfer);
    }

    let from_balance = *wallet_addresses.get(&from).unwrap();
    if from_balance < amount {
        println!("Insufficient funds!");
        return Err(TransferError::InsufficientFunds);
    }

    *wallet_addresses.get_mut(&from).unwrap() -= amount;
    *wallet_addresses.get_mut(&to).unwrap() += amount;

    Ok(())
}

fn mint(
    wallet_addresses: &mut HashMap<String, u64>,
    to: String,
    amount: u64,
) -> Result<(), TransferError> {
    if !wallet_addresses.contains_key(&to) {
        println!("Receiver not found!");
        return Err(TransferError::AccountNotFound);
    }

    if amount == 0 {
        println!("Amount must be greater than 0!");
        return Err(TransferError::InvalidAmount);
    }

    *wallet_addresses.get_mut(&to).unwrap() += amount;

    Ok(())
}

fn burn(
    wallet_addresses: &mut HashMap<String, u64>,
    from: String,
    amount: u64,
) -> Result<(), TransferError> {
    if !wallet_addresses.contains_key(&from) {
        println!("Sender not found!");
        return Err(TransferError::AccountNotFound);
    }

    if amount == 0 {
        println!("Amount must be greater than 0!");
        return Err(TransferError::InvalidAmount);
    }

    let from_balance = *wallet_addresses.get(&from).unwrap();
    if from_balance < amount {
        println!("Insufficient funds!");
        return Err(TransferError::InsufficientFunds);
    }

    *wallet_addresses.get_mut(&from).unwrap() -= amount;

    Ok(())
}

fn create_wallet(
    wallet_addresses: &mut HashMap<String, u64>,
    name: String,
) -> Result<(), CreateWalletError> {
    if wallet_addresses.contains_key(&name) {
        println!("Wallet already exists!");
        return Err(CreateWalletError::AccountAlreadyExists);
    }
    if name.len() < 3 {
        println!("Wallet name must be at least 3 characters long!");
        return Err(CreateWalletError::NameTooShort);
    }

    if name.len() > 12 {
        println!("Wallet name must be at most 12 characters long!");
        return Err(CreateWalletError::NameTooLong);
    }

    wallet_addresses.insert(name, 0);

    Ok(())
}

fn del_wallet(
    wallet_addresses: &mut HashMap<String, u64>,
    name: String,
) -> Result<(), DeleteWalletError> {
    if !wallet_addresses.contains_key(&name) {
        println!("Wallet not found!");
        return Err(DeleteWalletError::AccountNotFound);
    }

    if *wallet_addresses.get(&name).unwrap() > 0 {
        println!("Transfer all the token before deleting the wallet!");
        return Err(DeleteWalletError::NotEmpty);
    }

    wallet_addresses.remove(&name);

    Ok(())
}

fn get_balance(
    wallet_addresses: &mut HashMap<String, u64>,
    name: String,
) -> Result<u64, TransferError> {
    if !wallet_addresses.contains_key(&name) {
        println!("Wallet not found!");
        return Err(TransferError::AccountNotFound);
    }

    Ok(*wallet_addresses.get(&name).unwrap())
}

fn show_wallets(wallet_addresses: &mut HashMap<String, u64>) {
    for (name, balance) in wallet_addresses {
        println!("{}: {}", name, balance);
    }
}

fn main() {
    let mut wallet_addresses: HashMap<String, u64> = HashMap::new();

    loop {
        println!(">_______");
        println!("Available commands:");
        println!("1. Create wallet: create <name>");
        println!("2. Delete wallet: del <name>");
        println!("3. Get balance: balance <name>");
        println!("4. Mint tokens: mint <to> <amount>");
        println!("5. Burn tokens: burn <from> <amount>");
        println!("6. Transfer tokens: transfer <from> <to> <amount>");
        println!("7. Show all wallets: show");
        println!("8. Exit: exit");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "create" => {
                let name = parts[1].to_string();
                match create_wallet(&mut wallet_addresses, name) {
                    Ok(()) => println!("Wallet created successfully!"),
                    Err(_) => {}
                }
            }

            "del" => {
                let name = parts[1].to_string();
                match del_wallet(&mut wallet_addresses, name) {
                    Ok(()) => println!("Wallet deleted successfully!"),
                    Err(_) => {}
                }
            }

            "mint" => {
                let to = parts[1].to_string();
                let amount: u64 = match parts[2].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Invalid amount!");
                        continue;
                    }
                };

                match mint(&mut wallet_addresses, to, amount) {
                    Ok(()) => println!("Token minted successfully!"),
                    Err(_) => {}
                }
            }

            "burn" => {
                let from = parts[1].to_string();
                let amount: u64 = match parts[2].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Invalid amount!");
                        continue;
                    }
                };

                match burn(&mut wallet_addresses, from, amount) {
                    Ok(()) => println!("Token burned successfully!"),
                    Err(_) => {}
                }
            }

            "transfer" => {
                let from = parts[1].to_string();
                let to = parts[2].to_string();
                let amount: u64 = match parts[3].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Invalid amount!");
                        continue;
                    }
                };
                match transfer(&mut wallet_addresses, from, to, amount) {
                    Ok(()) => println!("Token transfered successfully!"),
                    Err(_) => {}
                }
            }

            "balance" => {
                let name = parts[1].to_string();
                match get_balance(&mut wallet_addresses, name) {
                    Ok(balance) => println!("Balance: {}", balance),
                    Err(_) => {}
                }
            }

            "show" => {
                show_wallets(&mut wallet_addresses);
            }

            "exit" => {
                println!("Goodbye!");
                break;
            }

            other => {
                println!("Unknown command: {}", other);
            }
        }
    }
}
