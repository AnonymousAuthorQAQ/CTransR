use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account: Arc<Mutex<i32>>) {
    let amount = 500000;
    let mut balance = account.lock().unwrap();
    *balance -= amount;
}

fn main() {
    let account = Arc::new(Mutex::new(1000000));

    let account_clone = Arc::clone(&account);
    let t1 = thread::spawn(move || withdraw(account_clone));

    let account_clone = Arc::clone(&account);
    let t2 = thread::spawn(move || withdraw(account_clone));

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", *account.lock().unwrap());
}