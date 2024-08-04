use std::sync::{Mutex, Arc};
use std::thread;

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
    let thread1 = thread::spawn(move || {
        withdraw(&balance_clone1);
    });

    let balance_clone2 = Arc::clone(&balance);
    let thread2 = thread::spawn(move || {
        withdraw(&balance_clone2);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Final account balance: {}", *balance.lock().unwrap());
}