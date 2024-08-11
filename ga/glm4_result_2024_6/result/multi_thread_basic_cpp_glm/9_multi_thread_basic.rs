use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn main() {
    let account_balance = AtomicI32::new(1000000);

    let handle1 = thread::spawn(move || {
        let mut balance = account_balance.load(Ordering::SeqCst);
        let mut amount = 500000;

        while amount > 0 && balance > 0 {
            balance -= 1;
            if account_balance.compare_and_swap(balance + 1, balance, Ordering::SeqCst) {
                amount -= 1;
            }
        }
    });

    let handle2 = thread::spawn(move || {
        let mut balance = account_balance.load(Ordering::SeqCst);
        let mut amount = 500000;

        while amount > 0 && balance > 0 {
            balance -= 1;
            if account_balance.compare_and_swap(balance + 1, balance, Ordering::SeqCst) {
                amount -= 1;
            }
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}