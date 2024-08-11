use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::Sender;

fn withdraw(tx: Sender<i32>, amount: i32) {
    let mut balance = tx.send(amount).unwrap();
    for _ in 0..amount {
        balance -= 1;
    }
    tx.send(balance).unwrap();
}

fn main() {
    let balance = Arc::new(Mutex::new(1000000));
    let (tx, rx) = std::sync::mpsc::channel();

    let balance_clone = Arc::clone(&balance);
    let tx_clone = tx.clone();
    thread::spawn(move || {
        let mut balance = *balance_clone.lock().unwrap();
        for _ in 0..500000 {
            balance -= 1;
        }
        tx_clone.send(balance).unwrap();
    });

    let balance_clone = Arc::clone(&balance);
    let tx_clone = tx.clone();
    thread::spawn(move || {
        let mut balance = *balance_clone.lock().unwrap();
        for _ in 0..500000 {
            balance -= 1;
        }
        tx_clone.send(balance).unwrap();
    });

    let final_balance = rx.recv().unwrap();
    println!("Final account balance: {}", final_balance);
}