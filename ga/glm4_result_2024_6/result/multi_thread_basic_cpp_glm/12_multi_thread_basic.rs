use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn withdraw(account_balance: &AtomicI32, amount: i32) {
    let mut remaining = amount;
    while remaining > 0 {
        if let Ok(balance) = account_balance.compare_and_swap(0, 0, Ordering::Acquire) {
            if balance > 0 {
                let decrement = if balance < remaining { balance } else { remaining };
                if account_balance.compare_and_swap(balance, balance - decrement, Ordering::AcqRel).is_ok() {
                    remaining -= decrement;
                }
            }
            break;
        }
    }
}

fn main() {
    let account_balance = AtomicI32::new(1000000);
    let amount = 500000;

    let handle1 = thread::spawn(move || withdraw(&account_balance, amount));
    let handle2 = thread::spawn(move || withdraw(&account_balance, amount));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}