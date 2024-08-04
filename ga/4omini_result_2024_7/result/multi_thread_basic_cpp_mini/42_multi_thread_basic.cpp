use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

const INITIAL_BALANCE: i32 = 1000000;
const WITHDRAW_AMOUNT: i32 = 500000;

fn withdraw(account_balance: Arc<AtomicI32>, amount: i32) {
    for _ in 0..amount {
        account_balance.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    let account_balance = Arc::new(AtomicI32::new(INITIAL_BALANCE));

    let balance_clone1 = Arc::clone(&account_balance);
    let balance_clone2 = Arc::clone(&account_balance);

    let t1 = thread::spawn(move || {
        withdraw(balance_clone1, WITHDRAW_AMOUNT);
    });

    let t2 = thread::spawn(move || {
        withdraw(balance_clone2, WITHDRAW_AMOUNT);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}