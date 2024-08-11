use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn withdraw(account_balance: &AtomicI32, amount: i32) {
    for _ in 0..amount {
        let mut balance = account_balance.load(Ordering::SeqCst);
        while balance > 0 {
            if balance == account_balance.compare_and_swap(balance, balance - 1, Ordering::SeqCst) {
                break;
            }
            balance = account_balance.load(Ordering::SeqCst);
        }
    }
}

fn main() {
    let account_balance = AtomicI32::new(1_000_000);

    let mut handles = vec![];
    for _ in 0..2 {
        let account_balance = &account_balance;
        handles.push(thread::spawn(move || {
            withdraw(account_balance, 500_000);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}