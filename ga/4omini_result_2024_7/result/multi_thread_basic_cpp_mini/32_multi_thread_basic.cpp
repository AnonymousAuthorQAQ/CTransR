use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

static INITIAL_BALANCE: i32 = 1_000_000;

fn withdraw(account_balance: Arc<AtomicI32>, amount: i32) {
    for _ in 0..amount {
        account_balance.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    let account_balance = Arc::new(AtomicI32::new(INITIAL_BALANCE));
    let withdraw_amount = 500_000;

    let balance_clone1 = Arc::clone(&account_balance);
    let handle1 = thread::spawn(move || {
        withdraw(balance_clone1, withdraw_amount);
    });

    let balance_clone2 = Arc::clone(&account_balance);
    let handle2 = thread::spawn(move || {
        withdraw(balance_clone2, withdraw_amount);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}