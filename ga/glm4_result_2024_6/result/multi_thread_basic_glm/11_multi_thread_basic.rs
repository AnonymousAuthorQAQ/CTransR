use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));  

    let handle1 = thread::spawn({
        let account_balance = Arc::clone(&account_balance);
        move || {
            let mut balance = account_balance.lock().unwrap();
            for _ in 0..500000 {
                *balance -= 1;
            }
        }
    });

    let handle2 = thread::spawn({
        let account_balance = Arc::clone(&account_balance);
        move || {
            let mut balance = account_balance.lock().unwrap();
            for _ in 0..500000 {
                *balance -= 1;
            }
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", *account_balance.lock().unwrap()); 
}