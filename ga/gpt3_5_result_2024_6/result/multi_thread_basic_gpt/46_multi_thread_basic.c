use std::sync::{Arc, Mutex};
use std::thread;

// Global shared variable representing account balance
static ACCOUNT_BALANCE: i32 = 1000000;

fn withdraw(account_balance: Arc<Mutex<i32>>) {
    let amount = 500000;
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(ACCOUNT_BALANCE));

    let account_balance1 = account_balance.clone();
    let thread1 = thread::spawn(move || {
        withdraw(account_balance1);
    });

    let account_balance2 = account_balance.clone();
    let thread2 = thread::spawn(move || {
        withdraw(account_balance2);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Final account balance: {}", *account_balance.lock().unwrap());
}