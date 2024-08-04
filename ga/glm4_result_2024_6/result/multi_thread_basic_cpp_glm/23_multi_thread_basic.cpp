use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn withdraw(account_balance: &AtomicI32, amount: i32) {
    for _ in 0..amount {
        account_balance.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    let account_balance = AtomicI32::new(1000000);

    let t1 = thread::spawn(move || {
        withdraw(&account_balance, 500000);
    });

    let t2 = thread::spawn(move || {
        withdraw(&account_balance, 500000);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}