use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

static ACCOUNT_BALANCE: AtomicUsize = AtomicUsize::new(1_000_000);

fn withdraw() {
    let amount = 500_000;  
    for _ in 0..amount {
        ACCOUNT_BALANCE.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    let t1 = thread::spawn(withdraw);
    let t2 = thread::spawn(withdraw);

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", ACCOUNT_BALANCE.load(Ordering::SeqCst));
}