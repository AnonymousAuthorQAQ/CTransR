use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_ACCOUNT_BALANCE: i32 = 1_000_000;
const WITHDRAWAL_AMOUNT: i32 = 500_000;

fn withdraw(account_balance: Arc<Mutex<i32>>) {
    for _ in 0..WITHDRAWAL_AMOUNT {
        // Lock the mutex to safely update the account_balance
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1; // Withdraw $1
    }
}

fn main() {
    // Create a shared account balance protected by a mutex
    let account_balance = Arc::new(Mutex::new(INITIAL_ACCOUNT_BALANCE));
    
    // Create two threads
    let account_balance1 = Arc::clone(&account_balance);
    let thread1 = thread::spawn(move || {
        withdraw(account_balance1);
    });

    let account_balance2 = Arc::clone(&account_balance);
    let thread2 = thread::spawn(move || {
        withdraw(account_balance2);
    });

    // Wait for both threads to complete
    thread1.join().unwrap();
    thread2.join().unwrap();

    // Output the final account balance
    let final_balance = *account_balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}