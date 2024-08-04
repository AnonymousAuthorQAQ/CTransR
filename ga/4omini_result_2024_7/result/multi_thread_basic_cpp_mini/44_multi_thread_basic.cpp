use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

struct Account {
    balance: AtomicI32,
}

impl Account {
    fn new(initial_balance: i32) -> Self {
        Account {
            balance: AtomicI32::new(initial_balance),
        }
    }

    fn withdraw(&self, amount: i32) {
        for _ in 0..amount {
            // Decrease the account balance atomically.
            self.balance.fetch_sub(1, Ordering::SeqCst);
        }
    }
}

fn main() {
    let account = Account::new(1_000_000); // Starting balance

    let account_clone = account.clone();
    let thread1 = thread::spawn(move || {
        account_clone.withdraw(500_000);
    });

    let account_clone2 = account.clone();
    let thread2 = thread::spawn(move || {
        account_clone2.withdraw(500_000);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    // Final account balance output
    println!("Final account balance: {}", account.balance.load(Ordering::SeqCst));
}

// Implement Clone trait for Account
impl Clone for Account {
    fn clone(&self) -> Self {
        Account {
            // In Rust, we can just consider the balance reference because we need separate access not reference
            balance: AtomicI32::new(self.balance.load(Ordering::SeqCst)),
        }
    }
}