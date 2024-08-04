use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn withdraw(balance: &AtomicI32, amount: i32) {
    for _ in 0..amount {
        balance.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    let account_balance = AtomicI32::new(1_000_000);

    let handle1 = thread::spawn(move || {
        withdraw(&account_balance, 500_000);
    });

    let handle2 = thread::spawn(move || {
        withdraw(&account_balance, 500_000);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}