use std::sync::{Arc, Mutex};
use std::thread;

// Global shared variable representing account balance
static mut ACCOUNT_BALANCE: i32 = 1000000;

fn withdraw() {
    let amount = 500000;
    for _ in 0..amount {
        unsafe {
            let mutex = &MUTEX;
            let mut account_balance = mutex.lock().unwrap();
            *account_balance -= 1;
        }
    }
}

// Mutex for protecting the account balance
lazy_static::lazy_static! {
    static ref MUTEX: Arc<Mutex<i32>> = Arc::new(Mutex::new(ACCOUNT_BALANCE));
}

fn main() {
    let mut threads = vec![];
    
    for _ in 0..2 {
        let thread = thread::spawn(|| {
            withdraw();
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    unsafe {
        println!("Final account balance: {}", ACCOUNT_BALANCE);
    }
}