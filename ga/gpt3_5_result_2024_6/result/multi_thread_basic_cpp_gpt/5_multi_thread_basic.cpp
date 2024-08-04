use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn withdraw(account_balance: &Arc<AtomicI32>) {
    let amount = 500000;
    for _ in 0..amount {
        account_balance.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    let account_balance = Arc::new(AtomicI32::new(1000000));

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

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}