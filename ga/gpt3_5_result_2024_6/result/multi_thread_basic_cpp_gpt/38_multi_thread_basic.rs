use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

static ACCOUNT_BALANCE: AtomicI32 = AtomicI32::new(1000000);

fn withdraw() {
    let amount = 500000;
    for _ in 0..amount {
        ACCOUNT_BALANCE.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    let balance_ref = Arc::new(ACCOUNT_BALANCE);

    let balance_ref1 = Arc::clone(&balance_ref);
    let t1 = thread::spawn(move || {
        withdraw();
    });

    let balance_ref2 = Arc::clone(&balance_ref);
    let t2 = thread::spawn(move || {
        withdraw();
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", balance_ref.load(Ordering::SeqCst));
}