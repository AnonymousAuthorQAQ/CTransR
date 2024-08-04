use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        // Lock the mutex to safely access the shared balance
        let mut account_balance = balance.lock().unwrap();
        *account_balance -= 1; // Decrement the balance by 1
    }
}

fn main() {
    let initial_balance = 1_000_000;
    
    // Create an Arc to allow shared ownership of the Mutex
    let balance = Arc::new(Mutex::new(initial_balance));

    // Specify the amount to withdraw in full
    let amount: i32 = 500_000;

    // Create two threads
    let balance_clone1 = Arc::clone(&balance);
    let handle1 = thread::spawn(move || {
        withdraw(balance_clone1, amount);
    });

    let balance_clone2 = Arc::clone(&balance);
    let handle2 = thread::spawn(move || {
        withdraw(balance_clone2, amount);
    });

    // Wait for both threads to complete
    handle1.join().unwrap();
    handle2.join().unwrap();

    // Print the final account balance
    let final_balance = *balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}