use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;

fn withdraw(account_balance: Arc<AtomicUsize>) {
    let amount = 500_000;  
    for _ in 0..amount {
        // Decrementing the value atomically
        account_balance.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    // An atomic variable initialized to 1_000_000 and wrapped in an Arc for shared ownership
    let account_balance = Arc::new(AtomicUsize::new(1_000_000));
    
    // Create cloneable references for each thread
    let acc_balance_clone1 = Arc::clone(&account_balance);
    let acc_balance_clone2 = Arc::clone(&account_balance);
    
    let t1 = thread::spawn(move || {
        withdraw(acc_balance_clone1);
    });

    let t2 = thread::spawn(move || {
        withdraw(acc_balance_clone2);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}