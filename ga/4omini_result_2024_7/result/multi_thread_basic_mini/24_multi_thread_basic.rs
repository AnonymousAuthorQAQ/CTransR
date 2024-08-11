use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000;
const WITHDRAWAL_AMOUNT: i32 = 500_000;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap();
        if *balance > 0 {
            *balance -= 1;
        }
    }
}

fn main() {
    // Shared account balance protected by a Mutex
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE));
    
    // Create two threads for concurrent withdrawal
    let account_balance_clone1 = Arc::clone(&account_balance);
    let thread1 = thread::spawn(move || {
        withdraw(account_balance_clone1, WITHDRAWAL_AMOUNT);
    });

    let account_balance_clone2 = Arc::clone(&account_balance);
    let thread2 = thread::spawn(move || {
        withdraw(account_balance_clone2, WITHDRAWAL_AMOUNT);
    });

    // Wait for both threads to complete
    thread1.join().unwrap();
    thread2.join().unwrap();

    // Print the final account balance
    let balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *balance);
}