use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_ACCOUNT_BALANCE: i32 = 1_000_000;
const WITHDRAW_AMOUNT: i32 = 500_000;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap(); // Lock the mutex to access the account balance
        *balance -= 1; // Decrement the account balance by 1
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(INITIAL_ACCOUNT_BALANCE));

    let account_balance_clone1 = Arc::clone(&account_balance);
    let handle1 = thread::spawn(move || {
        withdraw(account_balance_clone1, WITHDRAW_AMOUNT);
    });

    let account_balance_clone2 = Arc::clone(&account_balance);
    let handle2 = thread::spawn(move || {
        withdraw(account_balance_clone2, WITHDRAW_AMOUNT);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // Final account balance
    let final_balance = *account_balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}