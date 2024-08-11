use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000;
const WITHDRAW_AMOUNT: i32 = 500_000;

fn withdraw(balance: Arc<Mutex<i32>>) {
    for _ in 0..WITHDRAW_AMOUNT {
        let mut bal = balance.lock().unwrap(); // Lock the mutex to access and modify account_balance
        *bal -= 1; // Simulating withdrawal by decrementing the account balance by 1
    }
}

fn main() {
    let balance = Arc::new(Mutex::new(INITIAL_BALANCE)); // Create a new Mutex-protected balance shared across threads

    let balance_clone1 = Arc::clone(&balance);
    let handle1 = thread::spawn(move || withdraw(balance_clone1));

    let balance_clone2 = Arc::clone(&balance);
    let handle2 = thread::spawn(move || withdraw(balance_clone2));

    handle1.join().unwrap(); // Wait for thread 1 to finish
    handle2.join().unwrap(); // Wait for thread 2 to finish

    // Display the final account balance
    let final_balance = *balance.lock().unwrap(); // Lock the mutex to read the value
    println!("Final account balance: {}", final_balance); 
}