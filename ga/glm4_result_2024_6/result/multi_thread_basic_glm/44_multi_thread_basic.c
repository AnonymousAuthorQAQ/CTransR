use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account: Arc<Mutex<i32>>) {
    let amount = 500000;
    for _ in 0..amount {
        let mut balance = account.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    let account_balance: i32 = 1000000;
    let account = Arc::new(Mutex::new(account_balance));

    let account_clone1 = account.clone();
    let handle1 = thread::spawn(move || withdraw(account_clone1));

    let account_clone2 = account.clone();
    let handle2 = thread::spawn(move || withdraw(account_clone2));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", *account.lock().unwrap());
}