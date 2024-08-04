use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1_000_000;
const WITHDRAW_AMOUNT: i32 = 500_000;

fn withdraw(balance: Arc<Mutex<i32>>) {
    // Simulating the withdrawal process
    for _ in 0..WITHDRAW_AMOUNT {
        let mut locked_balance = balance.lock().unwrap();
        *locked_balance -= 1;  // Decrement the balance by 1
    }
}

fn main() {
    let balance = Arc::new(Mutex::new(INITIAL_BALANCE));

    // Cloning Arc to share the balance between threads
    let balance1 = Arc::clone(&balance);
    let balance2 = Arc::clone(&balance);

    // Creating threads for withdrawals
    let t1 = thread::spawn(move || withdraw(balance1));
    let t2 = thread::spawn(move || withdraw(balance2));

    // Waiting for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Printing the final account balance
    let final_balance = *balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}