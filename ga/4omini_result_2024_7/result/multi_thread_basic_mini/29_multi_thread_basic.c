use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        // Lock the mutex before accessing the shared account balance
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1; // Decrement the balance
    }
}

fn main() {
    let initial_balance = 1_000_000;
    let account_balance = Arc::new(Mutex::new(initial_balance));

    let amount_to_withdraw = 500_000;

    // Clone the Arc to pass to threads
    let account_balance1 = Arc::clone(&account_balance);
    let account_balance2 = Arc::clone(&account_balance);

    // Create two threads to withdraw money
    let t1 = thread::spawn(move || {
        withdraw(account_balance1, amount_to_withdraw);
    });

    let t2 = thread::spawn(move || {
        withdraw(account_balance2, amount_to_withdraw);
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    let final_balance = *account_balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}