use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn withdraw(account_balance: Arc<AtomicI32>, amount: i32) {
    for _ in 0..amount {
        account_balance.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    let account_balance = Arc::new(AtomicI32::new(1_000_000));
    let amount_to_withdraw = 500_000;

    let account_balance_clone1 = Arc::clone(&account_balance);
    let account_balance_clone2 = Arc::clone(&account_balance);

    let t1 = thread::spawn(move || withdraw(account_balance_clone1, amount_to_withdraw));
    let t2 = thread::spawn(move || withdraw(account_balance_clone2, amount_to_withdraw));

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::SeqCst));
}