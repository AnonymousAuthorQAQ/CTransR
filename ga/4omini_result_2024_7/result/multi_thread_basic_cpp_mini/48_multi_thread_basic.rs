use std::sync::{Arc, atomic::{AtomicI32, Ordering}};
use std::thread;

static INITIAL_BALANCE: i32 = 1000000;
static WITHDRAW_AMOUNT: i32 = 500000;

fn withdraw(account_balance: Arc<AtomicI32>) {
    for _ in 0..WITHDRAW_AMOUNT {
        // Atomically decrement the account balance
        account_balance.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    // Initialize an atomic integer for the account balance
    let account_balance = Arc::new(AtomicI32::new(INITIAL_BALANCE));

    // Create two threads for withdrawal
    let balance_clone_1 = Arc::clone(&account_balance);
    let t1 = thread::spawn(move || {
        withdraw(balance_clone_1);
    });

    let balance_clone_2 = Arc::clone(&account_balance);
    let t2 = thread::spawn(move || {
        withdraw(balance_clone_2);
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Output the final account balance
    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}