use std::thread;
use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
    let account_balance = AtomicI32::new(1000000);
    
    let mut handles = vec![];

    for _ in 0..2 {
        let account_balance = &account_balance;
        let handle = thread::spawn(move || {
            let mut amount = 500000;
            while amount > 0 {
                account_balance.fetch_sub(1, Ordering::SeqCst);
                amount -= 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}