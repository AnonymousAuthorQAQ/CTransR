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
    let t1_balance = Arc.clone(&account_balance);
    let t2_balance = Arc.clone(&account_balance);
    
    // Create threads for withdrawing
    let t1 = thread::spawn(move || withdraw(t1_balance, 500_000));
    let t2 = thread::spawn(move || withdraw(t2_balance, 500_000));

    // Wait for both threads to complete
    t1.join().unwrap();
    t2.join().unwrap();

    // Print final account balance
    let balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", *balance);
}