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
    let account_balance = Arc::new(ACCOUNT_BALANCE);
    
    let account_balance1 = Arc::clone(&account_balance);
    let thread1 = thread::spawn(move || {
        withdraw();
    });

    let account_balance2 = Arc::clone(&account_balance);
    let thread2 = thread::spawn(move || {
        withdraw();
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}