use std::sync::Arc;
use std::thread;

use std::sync::atomic::{AtomicUsize, Ordering};

fn withdraw(account_balance: &Arc<AtomicUsize>) {
    let amount = 500000;
    for _ in 0..amount {
        account_balance.fetch_sub(1, Ordering::Relaxed);
    }
}

fn main() {
    let account_balance = Arc::new(AtomicUsize::new(1000000));

    let account_balance_clone1 = account_balance.clone();
    let handle1 = thread::spawn(move || {
        withdraw(&account_balance_clone1);
    });

    let account_balance_clone2 = account_balance.clone();
    let handle2 = thread::spawn(move || {
        withdraw(&account_balance_clone2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::Relaxed));
}