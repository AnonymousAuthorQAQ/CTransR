use std::sync::{Arc, Mutex};
use std::thread;

// Global shared variable representing account balance
static mut ACCOUNT_BALANCE: i32 = 1000000;
static mut MUTEX: Option<Mutex<i32>> = None;

fn withdraw() {
    let amount = 500000;
    for _ in 0..amount {
        unsafe {
            if let Some(ref mutex) = MUTEX {
                let mut balance = mutex.lock().unwrap();
                *balance -= 1;
            }
        }
    }
}

fn main() {
    unsafe {
        MUTEX = Some(Mutex::new(ACCOUNT_BALANCE));
    }

    let handle1 = thread::spawn(|| {
        withdraw();
    });

    let handle2 = thread::spawn(|| {
        withdraw();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        if let Some(ref mutex) = MUTEX {
            let balance = mutex.lock().unwrap();
            println!("Final account balance: {}", *balance);
        }
    }
}