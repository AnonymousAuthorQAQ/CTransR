use std::thread;
use std::sync::{Mutex, Arc};

// Global shared variable representing account balance
static ACCOUNT_BALANCE: Mutex<i32> = Mutex::new(1000000);

fn withdraw() {
    let amount = 500000;
    for _ in 0..amount {
        let mut balance = ACCOUNT_BALANCE.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    let balance = Arc::new(ACCOUNT_BALANCE);

    let balance_clone1 = Arc::clone(&balance);
    let handle1 = thread::spawn(move || {
        withdraw(balance_clone1);
    });

    let balance_clone2 = Arc::clone(&balance);
    let handle2 = thread::spawn(move || {
        withdraw(balance_clone2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", *balance.lock().unwrap());
}