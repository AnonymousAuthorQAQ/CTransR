use std::sync::{Arc, atomic::AtomicI32};
use std::thread;

fn withdraw(account_balance: Arc<AtomicI32>) {
    let amount = 500000;
    for _ in 0..amount {
        account_balance.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }
}

fn main() {
    let account_balance = Arc::new(AtomicI32::new(1000000));

    let account_balance_clone = Arc::clone(&account_balance);
    let t1 = thread::spawn(move || withdraw(account_balance_clone));

    let account_balance_clone = Arc::clone(&account_balance);
    let t2 = thread::spawn(move || withdraw(account_balance_clone));

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(std::sync::atomic::Ordering::Relaxed));
}