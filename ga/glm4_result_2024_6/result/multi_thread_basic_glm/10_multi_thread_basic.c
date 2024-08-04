use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    let account_balance: Arc<Mutex<i32>> = Arc::new(Mutex::new(1000000));

    let mut handles = vec![];

    for _ in 0..2 {
        let account_balance = Arc::clone(&account_balance);
        let handle = thread::spawn(move || {
            let mut amount = 500000;
            while amount > 0 {
                let mut balance = account_balance.lock().unwrap();
                if *balance > 0 {
                    *balance -= 1;
                    amount -= 1;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final account balance: {}", *account_balance.lock().unwrap());
}