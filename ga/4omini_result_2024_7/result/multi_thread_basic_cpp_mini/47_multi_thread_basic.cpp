use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

static ACCOUNT_BALANCE: AtomicI32 = AtomicI32::new(1_000_000);

fn withdraw() {
    let amount = 500_000;
    for _ in 0..amount {
        ACCOUNT_BALANCE.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    let t1 = thread::spawn(withdraw);
    let t2 = thread::spawn(withdraw);

    t1.join().expect("Thread 1 panicked");
    t2.join().expect("Thread 2 panicked");

    println!("Final account balance: {}", ACCOUNT_BALANCE.load(Ordering::SeqCst));
}