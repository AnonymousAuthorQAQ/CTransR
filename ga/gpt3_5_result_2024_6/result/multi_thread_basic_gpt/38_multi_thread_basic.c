use std::thread;
use std::sync::{Arc, Mutex};

// Global shared variable representing account balance
static mut ACCOUNT_BALANCE: i32 = 1000000;
 
fn main() {
    // Create a mutex
    let mutex = Arc::new(Mutex::new(()));

    // Clone the mutex for each thread
    let mutex1 = mutex.clone();
    let mutex2 = mutex.clone();

    // Create threads and run withdraw function in each thread
    let handle1 = thread::spawn(move || {
        let amount = 500000;
        for _ in 0..amount {
            let _ = mutex1.lock().unwrap();
            unsafe {
                ACCOUNT_BALANCE -= 1;
            }
        }
    });

    let handle2 = thread::spawn(move || {
        let amount = 500000;
        for _ in 0..amount {
            let _ = mutex2.lock().unwrap();
            unsafe {
                ACCOUNT_BALANCE -= 1;
            }
        }
    });

    // Wait for threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        println!("Final account balance: {}", *ACCOUNT_BALANCE);
    }
}