use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        // Lock the account_balance to safely modify it
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    // Create an atomic balance using Arc and Mutex
    let account_balance = Arc::new(Mutex::new(1000000));

    let balance_clone1 = Arc::clone(&account_balance);
    let balance_clone2 = Arc::clone(&account_balance);

    // Spawn two threads to withdraw
    let t1 = thread::spawn(move || withdraw(balance_clone1, 500000));
    let t2 = thread::spawn(move || withdraw(balance_clone2, 500000));

    // Wait for threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *final_balance);
}