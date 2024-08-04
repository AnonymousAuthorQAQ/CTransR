use std::sync::{Arc, atomic::{AtomicI32, Ordering}};
use std::thread;

fn withdraw(account_balance: Arc<AtomicI32>) {
    let amount = 500000;  
    for _ in 0..amount {
        // We need to ensure that the decrements are atomic
        account_balance.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    let account_balance = Arc::new(AtomicI32::new(1000000));

    let account_balance_clone1 = Arc::clone(&account_balance);
    let account_balance_clone2 = Arc::clone(&account_balance);
    
    let t1 = thread::spawn(move || {
        withdraw(account_balance_clone1);
    });

    let t2 = thread::spawn(move || {
        withdraw(account_balance_clone2);
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}