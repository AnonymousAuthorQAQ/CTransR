use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::atomic::{AtomicI32, Ordering};

fn withdraw(balance: &Mutex<i32>, amount: i32) {
    let mut balance = balance.lock().unwrap();
    for _ in 0..amount {
        *balance -= 1;
    }
}

fn main() {
    let balance: Arc<Mutex<i32>> = Arc::new(Mutex::new(1000000));

    let balance_clone = Arc::clone(&balance);
    let handle1 = thread::spawn(move || {
        withdraw(&balance_clone, 500000);
    });

    let balance_clone = Arc::clone(&balance);
    let handle2 = thread::spawn(move || {
        withdraw(&balance_clone, 500000);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final account balance: {}", *balance.lock().unwrap());
}