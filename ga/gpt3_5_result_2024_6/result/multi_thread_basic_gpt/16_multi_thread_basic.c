use std::thread;
use std::sync::{Mutex, Arc};

// Global shared variable representing account balance
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
    // Creating a mutex to protect the account balance
    let mutex = Arc::new(Mutex::new(()));

    // Creating threads to withdraw from the account balance
    let mutex1 = mutex.clone();
    let handle1 = thread::spawn(move || {
        let _lock = mutex1.lock().unwrap();
        withdraw();
    });

    let mutex2 = mutex.clone();
    let handle2 = thread::spawn(move || {
        let _lock = mutex2.lock().unwrap();
        withdraw();
    });

    // Waiting for threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        println!("Final account balance: {}", ACCOUNT_BALANCE); 
    }
}