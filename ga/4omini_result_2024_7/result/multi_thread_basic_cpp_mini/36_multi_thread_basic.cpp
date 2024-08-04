use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap(); // Lock the mutex to safely access the balance
        *balance -= 1; // Decrement the balance by 1
    }
}

fn main() {
    let initial_balance = 1_000_000;
    let account_balance = Arc::new(Mutex::new(initial_balance)); // Create an Arc and Mutex to share state

    let account_balance_clone1 = Arc::clone(&account_balance);
    let account_balance_clone2 = Arc::clone(&account_balance);

    // Create two threads that will attempt to withdraw 500,000
    let t1 = thread::spawn(move || withdraw(account_balance_clone1, 500_000));
    let t2 = thread::spawn(move || withdraw(account_balance_clone2, 500_000));

    t1.join().unwrap(); // Wait for the first thread to finish
    t2.join().unwrap(); // Wait for the second thread to finish

    // Print the final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}