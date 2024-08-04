use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn withdraw(account_balance: Arc<AtomicI32>) {
    let amount = 500_000;  
    for _ in 0..amount {
        account_balance.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    // Initialize account balance to 1,000,000
    let account_balance = Arc::new(AtomicI32::new(1_000_000));

    // Create two threads
    let balance_clone1 = Arc::clone(&account_balance);
    let t1 = thread::spawn(move || {
        withdraw(balance_clone1);
    });

    let balance_clone2 = Arc::clone(&account_balance);
    let t2 = thread::spawn(move || {
        withdraw(balance_clone2);
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}