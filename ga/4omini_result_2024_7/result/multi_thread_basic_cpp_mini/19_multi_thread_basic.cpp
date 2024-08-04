use std::sync::{Arc, Mutex};
use std::thread;

struct Account {
    balance: Arc<Mutex<i32>>,
}

impl Account {
    fn new(initial_balance: i32) -> Self {
        Account {
            balance: Arc::new(Mutex::new(initial_balance)),
        }
    }

    fn withdraw(&self, amount: i32) {
        let mut balance = self.balance.lock().unwrap();
        *balance -= amount;
    }
}

fn main() {
    let account = Account::new(1_000_000);
    let account_clone = account.clone();

    // Create two threads for withdrawal
    let t1 = thread::spawn(move || {
        for _ in 0..500_000 {
            account.withdraw(1);
        }
    });

    let t2 = thread::spawn(move || {
        for _ in 0..500_000 {
            account_clone.withdraw(1);
        }
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    // Output the final account balance
    let final_balance = *account.balance.lock().unwrap();
    println!("Final account balance: {}", final_balance);
}