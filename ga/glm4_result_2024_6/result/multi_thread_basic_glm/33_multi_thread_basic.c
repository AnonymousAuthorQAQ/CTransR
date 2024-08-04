use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));

    let mut handles = vec![];

    for _ in 0..2 {
        let account_balance = Arc::clone(&account_balance);
        let handle = thread::spawn(move || {
            let mut balance = account_balance.lock().unwrap();
            for _ in 0..500000 {
                *balance -= 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final account balance: {}", account_balance.lock().unwrap());
}