use std::sync::{Arc, atomic::AtomicI32};
use std::thread;

fn withdraw(account: Arc<AtomicI32>, amount: i32) {
    for _ in 0..amount {
        account.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }
}

fn main() {
    let account_balance = Arc::new(AtomicI32::new(1000000));

    let account1 = Arc::clone(&account_balance);
    let handle1 = thread::spawn(move || {
        withdraw(account1, 500000);
    });

    let account2 = Arc::clone(&account_balance);
    let handle2 = thread::spawn(move || {
        withdraw(account2, 500000);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(std::sync::atomic::Ordering::SeqCst));
}