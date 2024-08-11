use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn withdraw(account_balance: Arc<AtomicI32>, amount: i32) {
    for _ in 0..amount {
        account_balance.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    let account_balance = Arc::new(AtomicI32::new(1_000_000));
    let withdrawal_amount = 500_000;
    
    let balance_clone1 = Arc::clone(&account_balance);
    let balance_clone2 = Arc::clone(&account_balance);

    let t1 = thread::spawn(move || {
        withdraw(balance_clone1, withdrawal_amount);
    });

    let t2 = thread::spawn(move || {
        withdraw(balance_clone2, withdrawal_amount);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}