use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn withdraw(account: &AtomicI32, amount: i32) {
    for _ in 0..amount {
        account.fetch_sub(1, Ordering::Relaxed);
    }
}

fn main() {
    let account_balance = AtomicI32::new(1000000);
    
    let t1 = thread::spawn({
        let account = &account_balance;
        move || withdraw(account, 500000)
    });
    
    let t2 = thread::spawn({
        let account = &account_balance;
        move || withdraw(account, 500000)
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", account_balance.load(Ordering::Relaxed));
}