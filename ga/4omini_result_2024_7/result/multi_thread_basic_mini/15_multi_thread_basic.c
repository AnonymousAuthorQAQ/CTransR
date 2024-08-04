use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000;
const WITHDRAW_AMOUNT: i32 = 500_000;

fn withdraw(balance: Arc<Mutex<i32>>) {
    for _ in 0..WITHDRAW_AMOUNT {
        // Lock the mutex to access the balance safely
        let mut account_balance = balance.lock().unwrap();
        *account_balance -= 1; // Withdraw 1 at a time
    }
}

fn main() {
    // Create a shared, thread-safe balance
    let account_balance = Arc::new(Mutex::new(INITIAL_BALANCE));

    // Create handles for threads
    let balance1 = Arc::clone(&account_balance);
    let balance2 = Arc::clone(&account_balance);

    // Create two threads
    let t1 = thread::spawn(move || withdraw(balance1));
    let t2 = thread::spawn(move || withdraw(balance2));

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}