use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

static INITIAL_BALANCE: i32 = 1_000_000;

fn withdraw(account_balance: Arc<AtomicI32>, amount: i32) {
    for _ in 0..amount {
        let mut current_balance = account_balance.load(Ordering::SeqCst);
        // Simulate a check to ensure the account balance does not go negative
        if current_balance > 0 {
            account_balance.store(current_balance - 1, Ordering::SeqCst);
        }
    }
}

fn main() {
    let account_balance = Arc::new(AtomicI32::new(INITIAL_BALANCE));
    let withdraw_amount: i32 = 500_000;

    // Create two threads for concurrent withdrawals
    let account_clone1 = Arc::clone(&account_balance);
    let t1 = thread::spawn(move || withdraw(account_clone1, withdraw_amount));

    let account_clone2 = Arc::clone(&account_balance);
    let t2 = thread::spawn(move || withdraw(account_clone2, withdraw_amount));

    // Wait for both threads to complete
    t1.join().unwrap();
    t2.join().unwrap();

    // Output the final account balance
    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}