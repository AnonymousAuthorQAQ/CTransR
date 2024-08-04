use std::sync::{Arc, Mutex};
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
    let handle1 = thread::spawn(|| {
        withdraw();
    });

    let handle2 = thread::spawn(|| {
        withdraw();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let final_balance = ACCOUNT_BALANCE.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}