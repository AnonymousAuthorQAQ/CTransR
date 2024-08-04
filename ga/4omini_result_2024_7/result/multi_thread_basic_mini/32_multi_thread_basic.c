use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut bal = balance.lock().unwrap();
        *bal -= 1;
    }
}

fn main() {
    // Shared global account balance initialized to 1000000
    let initial_balance = Arc::new(Mutex::new(1_000_000));

    // Create threads
    let balance_clone1 = Arc::clone(&initial_balance);
    let balance_clone2 = Arc::clone(&initial_balance);

    // Each thread withdraws 500000
    let amount_to_withdraw = 500_000;

    let t1 = thread::spawn(move || withdraw(balance_clone1, amount_to_withdraw));
    let t2 = thread::spawn(move || withdraw(balance_clone2, amount_to_withdraw));

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Get the final account balance
    let final_balance = *initial_balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}