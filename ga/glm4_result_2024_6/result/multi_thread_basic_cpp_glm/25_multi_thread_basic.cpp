use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn main() {
    let account_balance = AtomicI32::new(1_000_000);

    let handle1 = thread::spawn(|| {
        let amount = 500_000;
        for _ in 0..amount {
            account_balance.fetch_sub(1, Ordering::SeqCst);
        }
    });

    let handle2 = thread::spawn(|| {
        let amount = 500_000;
        for _ in 0..amount {
            account_balance.fetch_sub(1, Ordering::SeqCst);
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}