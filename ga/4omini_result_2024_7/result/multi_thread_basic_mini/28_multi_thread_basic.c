use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    // Global shared variable representing account balance
    let account_balance = Arc::new(Mutex::new(1_000_000));

    // Amount to withdraw
    let withdraw_amount = 500_000;

    // Cloning the Arc for thread ownership
    let account_balance_clone1 = Arc::clone(&account_balance);
    let account_balance_clone2 = Arc::clone(&account_balance);

    // Create two threads for withdrawal
    let thread1 = thread::spawn(move || {
        withdraw(account_balance_clone1, withdraw_amount);
    });
    
    let thread2 = thread::spawn(move || {
        withdraw(account_balance_clone2, withdraw_amount);
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    // Get the final account balance
    let final_balance = *account_balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}