use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Account {
    balance: Arc<Mutex<i32>>,
}

impl Account {
    fn new(initial_balance: i32) -> Account {
        Account {
            balance: Arc::new(Mutex::new(initial_balance)),
        }
    }
    
    fn withdraw(&self, amount: i32) {
        let mut balance = self.balance.lock().unwrap();
        for _ in 0..amount {
            *balance -= 1;
        }
    }

    fn get_balance(&self) -> i32 {
        let balance = self.balance.lock().unwrap();
        *balance
    }
}

fn main() {
    let account = Account::new(1_000_000);
    let account_clone = account.clone();
    
    let t1 = thread::spawn(move || {
        account_clone.withdraw(500_000);
    });
    
    let t2 = thread::spawn(move || {
        account.withdraw(500_000);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final account balance: {}", account.get_balance());
}