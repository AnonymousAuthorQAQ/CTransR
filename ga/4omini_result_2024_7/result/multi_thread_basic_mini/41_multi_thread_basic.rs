use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    let mut balance = account_balance.lock().unwrap();
    for _ in 0..amount {
        // Decrement the account balance. 
        // Ensures there is enough balance to deduct from or could add checks here.
        *balance -= 1;
    }
}

fn main() {
    // Global shared variable representing account balance
    let account_balance = Arc::new(Mutex::new(1_000_000));

    let withdraw_amount = 500_000;

    // Clone the Arc for each thread
    let balance_clone_1 = Arc::clone(&account_balance);
    let balance_clone_2 = Arc::clone(&account_balance);

    let t1 = thread::spawn(move || {
        withdraw(balance_clone_1, withdraw_amount);
    });

    let t2 = thread::spawn(move || {
        withdraw(balance_clone_2, withdraw_amount);
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print final account balance
    println!("Final account balance: {}", *account_balance.lock().unwrap());
}