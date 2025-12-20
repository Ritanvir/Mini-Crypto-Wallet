use std::io;


struct Wallet {
    id: u32,
    name: String,
    coin_type: String, 
    balance: f64,
}

fn main() {
    let mut wallets: Vec<Wallet> = Vec::new();
    let mut next_id: u32 = 1;

    loop {
        println!("\n=== Mini Crypto Wallet ===");
        println!("1) Create new wallet");
        println!("2) Show all wallets");
        println!("3) Deposit");
        println!("4) Withdraw");
        println!("5) Exit");
        println!("6) Delete wallet"); 
        println!("Choose an option: ");

        let choice = read_u32();

        match choice {
            1 => {
                create_wallet(&mut wallets, &mut next_id);
            }
            2 => {
                show_wallets(&wallets);
            }
            3 => {
                deposit(&mut wallets);
            }
            4 => {
                withdraw(&mut wallets);
            }
            5 => {
                println!("Goodbye!");
                break;
            }
            6 => {
                delete_wallet(&mut wallets); 
            }
            _ => {
                println!("Invalid choice, try again.");
            }
        }
    }
}

// ---------- Helper input functions ---------- //

fn read_line_trimmed() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn read_u32() -> u32 {
    loop {
        let input = read_line_trimmed();
        match input.parse::<u32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid number:");
            }
        }
    }
}

fn read_f64() -> f64 {
    loop {
        let input = read_line_trimmed();
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid amount (number):");
            }
        }
    }
}

// ---------- Actions ---------- //

fn create_wallet(wallets: &mut Vec<Wallet>, next_id: &mut u32) {
    println!("Enter wallet name: ");
    let name = read_line_trimmed();

    println!("Enter coin type (e.g. SOL, ETH, BTC): ");
    let coin_type = read_line_trimmed();

    let wallet = Wallet {
        id: *next_id,
        name,
        coin_type,
        balance: 0.0,
    };

    wallets.push(wallet);
    println!("Wallet created with ID: {}", next_id);

    *next_id += 1;
}

fn show_wallets(wallets: &Vec<Wallet>) {
    if wallets.is_empty() {
        println!("No wallets found.");
        return;
    }

    println!("\n--- Wallets ---");
    for w in wallets {
        println!(
            "ID: {} | Name: {} | Coin: {} | Balance: {:.4}",
            w.id, w.name, w.coin_type, w.balance
        );
    }
}

fn find_wallet_mut<'a>(wallets: &'a mut Vec<Wallet>, id: u32) -> Option<&'a mut Wallet> {
    for w in wallets.iter_mut() {
        if w.id == id {
            return Some(w);
        }
    }
    None
}

fn deposit(wallets: &mut Vec<Wallet>) {
    if wallets.is_empty() {
        println!("No wallets available. Create one first.");
        return;
    }

    println!("Enter wallet ID to deposit into: ");
    let id = read_u32();

    match find_wallet_mut(wallets, id) {
        Some(w) => {
            println!(
                "Enter amount to deposit into {} ({}) :",
                w.name, w.coin_type
            );
            let amount = read_f64();
            if amount <= 0.0 {
                println!("Amount must be positive.");
                return;
            }
            w.balance += amount;
            println!(
                "Deposited {:.4} {}. New balance: {:.4} {}",
                amount, w.coin_type, w.balance, w.coin_type
            );
        }
        None => {
            println!("Wallet with ID {} not found.", id);
        }
    }
}

fn withdraw(wallets: &mut Vec<Wallet>) {
    if wallets.is_empty() {
        println!("No wallets available. Create one first.");
        return;
    }

    println!("Enter wallet ID to withdraw from: ");
    let id = read_u32();

    match find_wallet_mut(wallets, id) {
        Some(w) => {
            println!(
                "Enter amount to withdraw from {} ({}) :",
                w.name, w.coin_type
            );
            let amount = read_f64();
            if amount <= 0.0 {
                println!("Amount must be positive.");
                return;
            }
            if amount > w.balance {
                println!(
                    "Not enough balance. Current balance: {:.4} {}",
                    w.balance, w.coin_type
                );
                return;
            }
            w.balance -= amount;
            println!(
                "Withdrew {:.4} {}. New balance: {:.4} {}",
                amount, w.coin_type, w.balance, w.coin_type
            );
        }
        None => {
            println!("Wallet with ID {} not found.", id);
        }
    }
}

// ----------  Wallet delete ---------- //

fn delete_wallet(wallets: &mut Vec<Wallet>) {
    if wallets.is_empty() {
        println!("No wallets to delete.");
        return;
    }

    println!("Enter wallet ID to delete: ");
    let id = read_u32();

    let original_len = wallets.len();

    
    wallets.retain(|w| w.id != id);

    if wallets.len() < original_len {
        println!("Wallet with ID {} deleted.", id);
    } else {
        println!("Wallet with ID {} not found.", id);
    }
}
