use std::sync::{Arc, Mutex};
use std::thread;
use std::cmp;

fn main() {
    // Create an Arc<Mutex<i32>> to allow shared ownership with interior mutability.
    let account_balance = Arc::new(Mutex::new(1000000));

    // Function to perform the withdrawal.
    fn withdraw(account: &Arc<Mutex<i32>>, amount: i32) {
        let mut balance = account.lock().unwrap();
        for _ in 0..amount {
            *balance = cmp::max(0, *balance - 1);
        }
    }

    // Create two threads that perform the withdrawal.
    let account_balance_clone = Arc::clone(&account_balance);
    let handle1 = thread::spawn(move || {
        withdraw(&account_balance_clone, 500000);
    });

    let account_balance_clone = Arc::clone(&account_balance);
    let handle2 = thread::spawn(move || {
        withdraw(&account_balance_clone, 500000);
    });

    // Wait for the threads to complete.
    handle1.join().unwrap();
    handle2.join().unwrap();

    // Print the final account balance.
    println!("Final account balance: {}", *account_balance.lock().unwrap());
}