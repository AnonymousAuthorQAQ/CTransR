use std::thread;
use std::sync::{Mutex, Arc};

// Global shared variable representing account balance
let account_balance = Arc::new(Mutex::new(1000000));

fn withdraw(amount: i32, balance: Arc<Mutex<i32>>) {
    for _ in 0..amount {
        let mut bal = balance.lock().unwrap();
        *bal -= 1;
    }
}

fn main() {
    let balance = Arc::clone(&account_balance);
    
    let t1 = thread::spawn(move || withdraw(500000, balance));
    let balance = Arc::clone(&account_balance);
    let t2 = thread::spawn(move || withdraw(500000, balance));
    
    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", *account_balance.lock().unwrap());
}