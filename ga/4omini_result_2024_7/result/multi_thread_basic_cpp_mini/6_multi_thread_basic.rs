use std::sync::{Arc, Mutex};
use std::thread;

struct Account {
    balance: Mutex<i32>,
}

impl Account {
    fn withdraw(&self, amount: i32) {
        let mut balance = self.balance.lock().unwrap();
        *balance -= amount;
    }
}

fn main() {
    let account = Arc::new(Account {
        balance: Mutex::new(1_000_000),
    });

    let account_clone1 = Arc::clone(&account);
    let account_clone2 = Arc::clone(&account);

    let t1 = thread::spawn(move || {
        account_clone1.withdraw(500_000);
    });

    let t2 = thread::spawn(move || {
        account_clone2.withdraw(500_000);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {:?}", *account.balance.lock().unwrap());
}