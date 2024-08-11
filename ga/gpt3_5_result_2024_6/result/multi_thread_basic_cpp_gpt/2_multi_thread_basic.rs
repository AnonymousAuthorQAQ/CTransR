use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: &Arc<Mutex<i32>>) {
    let mut balance = account_balance.lock().unwrap();
    for _ in 0..500000 {
        *balance -= 1;
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1000000));

    let account_balance1 = Arc::clone(&account_balance);
    let thread1 = thread::spawn(move || {
        withdraw(&account_balance1);
    });

    let account_balance2 = Arc::clone(&account_balance);
    let thread2 = thread::spawn(move || {
        withdraw(&account_balance2);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    let balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *balance);
}