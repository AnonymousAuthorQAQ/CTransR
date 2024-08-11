use std::sync::{Arc, atomic::{AtomicI32, Ordering}};
use std::thread;

fn withdraw(account: Arc<AtomicI32>, amount: i32) {
    for _ in 0..amount {
        account.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    let account_balance = Arc::new(AtomicI32::new(1000000));
    
    let account_balance_clone = account_balance.clone();
    let t1 = thread::spawn(move || {
        withdraw(account_balance_clone, 500000);
    });

    let account_balance_clone = account_balance.clone();
    let t2 = thread::spawn(move || {
        withdraw(account_balance_clone, 500000);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}