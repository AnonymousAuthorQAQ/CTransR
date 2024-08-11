use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1; // Decrease account balance by 1
    }
}

fn main() {
    // Initial account balance
    let initial_balance = 1_000_000;
    
    // Arc allows multiple ownership of the account balance
    let account_balance = Arc::new(Mutex::new(initial_balance));
    
    let withdraw_amount = 500_000;

    // Create threads
    let account_balance_clone1 = Arc::clone(&account_balance);
    let handle1 = thread::spawn(move || withdraw(account_balance_clone1, withdraw_amount));

    let account_balance_clone2 = Arc::clone(&account_balance);
    let handle2 = thread::spawn(move || withdraw(account_balance_clone2, withdraw_amount));

    // Wait for threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();

    // Print final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}