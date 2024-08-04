use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        // Lock the mutex before modifying the balance
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    // Create a shared account balance wrapped in an Arc and Mutex
    let account_balance = Arc::new(Mutex::new(1_000_000));

    let amount_to_withdraw = 500_000;

    // Clone the Arc for each thread
    let account_balance_t1 = Arc::clone(&account_balance);
    let account_balance_t2 = Arc::clone(&account_balance);

    // Create two threads to perform the withdrawal
    let t1 = thread::spawn(move || {
        withdraw(account_balance_t1, amount_to_withdraw);
    });

    let t2 = thread::spawn(move || {
        withdraw(account_balance_t2, amount_to_withdraw);
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Access the final balance
    let final_balance = *account_balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}