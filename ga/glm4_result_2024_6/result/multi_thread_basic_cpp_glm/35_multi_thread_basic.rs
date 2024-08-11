use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let initial_balance = 1000000;
    let account_balance = Arc::new(Mutex::new(initial_balance));

    let mut threads = Vec::new();

    for _ in 0..2 {
        let balance = Arc::clone(&account_balance);
        let handle = thread::spawn(move || {
            let mut balance = balance.lock().unwrap();
            for _ in 0..500000 {
                *balance -= 1;
            }
        });
        threads.push(handle);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Final account balance: {}", *account_balance.lock().unwrap());
}