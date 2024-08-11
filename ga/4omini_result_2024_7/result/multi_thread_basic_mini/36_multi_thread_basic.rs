use std::sync::{Arc, Mutex};
use std::thread;

// Global shared variable representing account balance
const INITIAL_BALANCE: i32 = 1_000_000;
const WITHDRAWAL_AMOUNT: i32 = 500_000;

fn withdraw(balance: Arc<Mutex<i32>>) {
    for _ in 0..WITHDRAWAL_AMOUNT {
        let mut acc_balance = balance.lock().unwrap();
        *acc_balance -= 1; // Decrementing the shared balance
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE));

    let balance1 = Arc::clone(&account_balance);
    let balance2 = Arc::clone(&account_balance);

    // Creating two threads to perform the withdrawals
    let t1 = thread::spawn(move || withdraw(balance1));
    let t2 = thread::spawn(move || withdraw(balance2));

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Get the final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}