use std::sync::{Arc, Mutex};
use std::thread;

fn withdraw(account_balance: Arc<Mutex<i32>>, amount: i32) {
    for _ in 0..amount {
        let mut balance = account_balance.lock().unwrap();
        *balance -= 1;
    }
}

fn main() {
    let account_balance = Arc::new(Mutex::new(1_000_000)); // Initial account balance

    let account_clone1 = Arc::clone(&account_balance);
    let account_clone2 = Arc::clone(&account_balance);

    let t1 = thread::spawn(move || {
        withdraw(account_clone1, 500_000);
    });

    let t2 = thread::spawn(move || {
        withdraw(account_clone2, 500_000);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    let final_balance = account_balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}