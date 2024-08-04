use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000; // Initial account balance
const WITHDRAW_AMOUNT: i32 = 500_000; // Amount to withdraw

fn withdraw(balance: Arc<Mutex<i32>>, _amount: i32) {
    let amount = _amount;
    for _ in 0..amount {
        let mut balance = balance.lock().unwrap(); // Lock the mutex
        *balance -= 1; // Decrement the balance
    }
}

fn main() {
    let balance = Arc::new(Mutex::new(INITIAL_BALANCE)); // Shared and protected balance

    // Cloning the Arc to share with threads
    let balance1 = Arc::clone(&balance);
    let balance2 = Arc::clone(&balance);

    // Create two threads that attempt to withdraw simultaneously
    let t1 = thread::spawn(move || {
        withdraw(balance1, WITHDRAW_AMOUNT);
    });

    let t2 = thread::spawn(move || {
        withdraw(balance2, WITHDRAW_AMOUNT);
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Final balance report
    println!("Final account balance: {}", *balance.lock().unwrap());

    // No need to manually destroy mutex in Rust; it is dropped automatically
}