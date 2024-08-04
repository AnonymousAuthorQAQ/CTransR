use std::sync::{Arc, Mutex};
use std::thread;

struct BankAccount {
    balance: Mutex<i32>,
}

impl BankAccount {
    fn new(initial_balance: i32) -> Self {
        BankAccount {
            balance: Mutex::new(initial_balance),
        }
    }

    fn withdraw(&self, amount: i32) {
        // Simulate a loop of decrementing the balance by one for illustration (not efficient)
        for _ in 0..amount {
            // Lock the mutex to access and modify the balance
            let mut balance = self.balance.lock().unwrap();
            *balance -= 1;
        }
    }

    fn get_balance(&self) -> i32 {
        let balance = self.balance.lock().unwrap();
        *balance
    }
}

fn main() {
    let account = Arc::new(BankAccount::new(1_000_000));

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

    println!("Final account balance: {}", account.get_balance());
}