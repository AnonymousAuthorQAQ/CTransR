use std::thread;
use std::sync::{Mutex, Arc};

// Global shared variable representing account balance
static mut ACCOUNT_BALANCE: i32 = 1000000;
static MUTEX: Mutex<()> = Mutex::new(());

fn withdraw() {
    let amount = 500000;
    for _ in 0..amount {
        unsafe {
            let _lock = MUTEX.lock().unwrap();
            ACCOUNT_BALANCE -= 1;
        }
    }
}

fn main() {
    let mutex = Arc::new(Mutex::new(()));

    let mutex1 = Arc::clone(&mutex);
    let handle1 = thread::spawn(move || {
        withdraw();
    });

    let mutex2 = Arc::clone(&mutex);
    let handle2 = thread::spawn(move || {
        withdraw();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe {
        println!("Final account balance: {}", ACCOUNT_BALANCE);
    }
}