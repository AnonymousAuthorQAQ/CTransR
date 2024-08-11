use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

static ACCOUNT_BALANCE: AtomicI32 = AtomicI32::new(1_000_000);

fn withdraw() {
    let amount = 500_000;  
    for _ in 0..amount {
        // Use fetch_sub to safely decrement the atomic variable
        ACCOUNT_BALANCE.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    let t1 = thread::spawn(withdraw);
    let t2 = thread::spawn(withdraw);

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Print the final account balance
    println!("Final account balance: {}", ACCOUNT_BALANCE.load(Ordering::SeqCst));
}