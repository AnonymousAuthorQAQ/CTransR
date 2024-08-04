use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        // Lock the mutex to safely access the account balance
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1; // Decrement the balance
    }
}

fn main() {
    // Initial balance
    let initial_balance = 1_000_000;
    let account_balance = Arc::new(Mutex::new(initial_balance));

    let withdraw_amount = 500_000;
    
    let account_balance_clone1 = Arc::clone(&account_balance);
    let account_balance_clone2 = Arc::clone(&account_balance);

    // Create the first thread
    let t1 = thread::spawn(move || {
        withdraw(account_balance_clone1, withdraw_amount);
    });

    // Create the second thread
    let t2 = thread::spawn(move || {
        withdraw(account_balance_clone2, withdraw_amount);
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    let final_balance = *account_balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}