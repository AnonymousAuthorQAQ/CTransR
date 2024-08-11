use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn main() {
    let account_balance = AtomicI32::new(1_000_000);

    let t1 = thread::spawn(move || {
        let mut amount = 500_000;
        while amount > 0 {
            account_balance.fetch_sub(1, Ordering::SeqCst);
            amount -= 1;
        }
    });

    let t2 = thread::spawn(move || {
        let mut amount = 500_000;
        while amount > 0 {
            account_balance.fetch_sub(1, Ordering::SeqCst);
            amount -= 1;
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}