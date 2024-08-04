use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    // Initialize account balance with 1,000,000
    let account_balance = Arc::new(Mutex::new(1_000_000));

    // Create a clone of the Arc for each thread
    let balance_clone1 = Arc::clone(&account_balance);
    let balance_clone2 = Arc::clone(&account_balance);

    // Define the withdrawal function
    let withdraw = |balance: Arc<Mutex<i32>>, amount: i32| {
        // Attempt to withdraw the specified amount
        for _ in 0..amount {
            let mut balance_lock = balance.lock().unwrap();
            *balance_lock -= 1;
        }
    };

    // Start two threads to withdraw 500,000 each
    let t1 = thread::spawn(move || withdraw(balance_clone1, 500_000));
    let t2 = thread::spawn(move || withdraw(balance_clone2, 500_000));

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}