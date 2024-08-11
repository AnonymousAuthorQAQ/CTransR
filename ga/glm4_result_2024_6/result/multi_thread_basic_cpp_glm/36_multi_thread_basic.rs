use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn main() {
    let account_balance = AtomicI32::new(1_000_000);

    let withdraw = || {
        let amount = 500_000;
        for _ in 0..amount {
            account_balance.fetch_sub(1, Ordering::Relaxed);
        }
    };

    let t1 = thread::spawn(withdraw);
    let t2 = thread::spawn(withdraw);

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::Relaxed));
}