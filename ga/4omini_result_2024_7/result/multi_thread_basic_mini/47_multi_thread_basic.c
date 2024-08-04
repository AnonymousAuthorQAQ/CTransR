use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000;  // Initial account balance
const WITHDRAW_AMOUNT: i32 = 500_000;     // Withdrawal amount

fn withdraw(balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        // Lock the mutex to safely access and modify the balance
        let mut account_balance = balance.lock().unwrap();
        *account_balance -= 1;  // Decrement the balance by 1
    }
}

fn main() {
    let balance = Arc::new(Mutex::new(INITIAL_BALANCE));  // Create an Arc to share the balance across threads
    
    let balance_clone1 = Arc::clone(&balance);
    let handle1 = thread::spawn(move || {
        withdraw(balance_clone1, WITHDRAW_AMOUNT);
    });

    let balance_clone2 = Arc::clone(&balance);
    let handle2 = thread::spawn(move || {
        withdraw(balance_clone2, WITHDRAW_AMOUNT);
    });

    // Wait for both threads to complete
    handle1.join().unwrap();
    handle2.join().unwrap();

    // Print the final account balance
    let final_balance = balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}