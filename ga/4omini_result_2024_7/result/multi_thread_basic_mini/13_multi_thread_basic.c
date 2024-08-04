use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1_000_000)); // Initial account balance

    let amount_to_withdraw = 500_000;

    let account_balance_clone1 = Arc::clone(&account_balance);
    let thread1 = thread::spawn(move || {
        withdraw(account_balance_clone1, amount_to_withdraw);
    });

    let account_balance_clone2 = Arc::clone(&account_balance);
    let thread2 = thread::spawn(move || {
        withdraw(account_balance_clone2, amount_to_withdraw);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    // Display the final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}