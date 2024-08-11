use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000;
const WITHDRAW_AMOUNT: i32 = 500_000;

fn withdraw(account_balance: Arc<Mutex<i32>>) {
    for _ in 0..WITHDRAW_AMOUNT {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1; // Decrease balance by 1 for each iteration
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE));

    let balance_clone1 = Arc::clone(&account_balance);
    let balance_clone2 = Arc::clone(&account_balance);

    let t1 = thread::spawn(move || {
        withdraw(balance_clone1);
    });

    let t2 = thread::spawn(move || {
        withdraw(balance_clone2);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}