use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn main() {
    let account_balance = AtomicI32::new(1_000_000);

    let handle1 = thread::spawn(move || {
        let mut amount = 500_000;
        while amount > 0 {
            account_balance.fetch_sub(1, Ordering::SeqCst);
            amount -= 1;
        }
    });

    let handle2 = thread::spawn(move || {
        let mut amount = 500_000;
        while amount > 0 {
            account_balance.fetch_sub(1, Ordering::SeqCst);
            amount -= 1;
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}