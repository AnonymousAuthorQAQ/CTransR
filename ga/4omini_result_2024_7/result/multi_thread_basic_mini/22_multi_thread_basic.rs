use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000;
const WITHDRAW_AMOUNT: i32 = 500_000;

fn withdraw(balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut account_balance = balance.lock().unwrap();
        *account_balance -= 1;
    }
}

fn main() {
    let balance = Arc::new(Mutex::new(INITIAL_BALANCE));

    // Create two threads for withdrawals
    let balance1 = Arc::clone(&balance);
    let t1 = thread::spawn(move || {
        withdraw(balance1, WITHDRAW_AMOUNT);
    });

    let balance2 = Arc::clone(&balance);
    let t2 = thread::spawn(move || {
        withdraw(balance2, WITHDRAW_AMOUNT);
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    let final_balance = *balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}