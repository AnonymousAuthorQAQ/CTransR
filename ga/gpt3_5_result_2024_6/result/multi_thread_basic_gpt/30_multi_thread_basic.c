use std::sync::{Arc, Mutex};
use std::thread;

// Shared variable representing account balance
static mut ACCOUNT_BALANCE: i32 = 1000000;

fn withdraw() {
    let amount = 500000;
    for _ in 0..amount {
        unsafe {
            ACCOUNT_BALANCE -= 1;
        }
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(ACCOUNT_BALANCE));

    let account_balance_clone1 = account_balance.clone();
    let handle1 = thread::spawn(move || {
        for _ in 0..2 {
            withdraw();
        }
        let balance = account_balance_clone1.lock().unwrap();
        println!("Thread 1: Final account balance: {}", *balance);
    });

    let account_balance_clone2 = account_balance.clone();
    let handle2 = thread::spawn(move || {
        for _ in 0..2 {
            withdraw();
        }
        let balance = account_balance_clone2.lock().unwrap();
        println!("Thread 2: Final account balance: {}", *balance);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Main Thread: Final account balance: {}", unsafe { ACCOUNT_BALANCE });
}