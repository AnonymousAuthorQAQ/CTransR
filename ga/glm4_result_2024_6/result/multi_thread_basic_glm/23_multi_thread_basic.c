use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let account_balance: Arc<Mutex<i32>> = Arc::new(Mutex::new(1000000));
    let (sender, receiver) = channel();

    let account_balance1 = Arc::clone(&account_balance);
    let sender1 = sender.clone();
    thread::spawn(move || {
        for _ in 0..500000 {
            let mut balance = account_balance1.lock().unwrap();
            *balance -= 1;
        }
        sender1.send(()).unwrap();
    });

    let account_balance2 = Arc::clone(&account_balance);
    let sender2 = sender;
    thread::spawn(move || {
        for _ in 0..500000 {
            let mut balance = account_balance2.lock().unwrap();
            *balance -= 1;
        }
        sender2.send(()).unwrap();
    });

    receiver.recv().unwrap();
    receiver.recv().unwrap();

    println!("Final account balance: {}", *account_balance.lock().unwrap());
}