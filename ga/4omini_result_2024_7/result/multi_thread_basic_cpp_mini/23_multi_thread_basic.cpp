use std::sync::{Arc, atomic::{AtomicI32, Ordering}};
use std::thread;

fn withdraw(account_balance: Arc<AtomicI32>, amount: i32) {
    for _ in 0..amount {
        // Use the fetch_sub method for atomic subtraction
        account_balance.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    // Initialize the account balance to 1,000,000
    let account_balance = Arc::new(AtomicI32::new(1_000_000));
    let amount_to_withdraw: i32 = 500_000; 

    let balance_clone1 = Arc::clone(&account_balance);
    let balance_clone2 = Arc::clone(&account_balance);
    
    // Spawn two threads to perform withdrawals
    let t1 = thread::spawn(move || {
        withdraw(balance_clone1, amount_to_withdraw);
    });
    let t2 = thread::spawn(move || {
        withdraw(balance_clone2, amount_to_withdraw);
    });

    // Wait for both threads to complete
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}