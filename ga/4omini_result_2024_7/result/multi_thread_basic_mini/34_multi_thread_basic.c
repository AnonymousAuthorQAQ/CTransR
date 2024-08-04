use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000;
const WITHDRAW_AMOUNT: i32 = 500_000;

fn withdraw(balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut bal = balance.lock().unwrap();
        *bal -= 1; // Decrement the balance by 1
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE));

    let balance_clone1 = Arc::clone(&account_balance);
    let handle1 = thread::spawn(move || {
        withdraw(balance_clone1, WITHDRAW_AMOUNT);
    });

    let balance_clone2 = Arc::clone(&account_balance);
    let handle2 = thread::spawn(move || {
        withdraw(balance_clone2, WITHDRAW_AMOUNT);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let final_balance = *account_balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}