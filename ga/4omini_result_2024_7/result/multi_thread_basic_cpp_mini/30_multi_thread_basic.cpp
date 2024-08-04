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
    let thread1 = thread::spawn(withdraw);
    let thread2 = thread::spawn(withdraw);

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Final account balance: {}", ACCOUNT_BALANCE.load(Ordering::SeqCst));
}