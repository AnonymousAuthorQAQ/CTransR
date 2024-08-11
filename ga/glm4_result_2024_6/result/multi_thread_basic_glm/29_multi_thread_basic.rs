use std::thread;
use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicU32, Ordering};

fn main() {
    let account_balance = Arc::new(AtomicU32::new(1000000));
    let mut handles = vec![];

    for _ in 0..2 {
        let account_balance = Arc::clone(&account_balance);
        let handle = thread::spawn(move || {
            let amount = 500000;
            for _ in 0..amount {
                account_balance.fetch_sub(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}