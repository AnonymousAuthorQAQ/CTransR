use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Account {
    balance: i32,
}

fn withdraw(account: Arc<Mutex<Account>>, amount: i32) {
    let mut account_lock = account.lock().unwrap();
    for _ in 0..amount {
        // Only decrease the balance if there's enough to withdraw
        if account_lock.balance > 0 {
            account_lock.balance -= 1;
        } else {
            break; // Exit if there are no funds left
        }
    }
}

fn main() {
    let initial_balance = 1_000_000;
    let account = Arc::new(Mutex::new(Account { balance: initial_balance }));

    let account_clone1 = Arc::clone(&account);
    let account_clone2 = Arc::clone(&account);

    let t1 = thread::spawn(move || withdraw(account_clone1, 500_000));
    let t2 = thread::spawn(move || withdraw(account_clone2, 500_000));

    t1.join().unwrap();
    t2.join().unwrap();

    let final_balance = account.lock().unwrap().balance;
    println!("Final account balance: {}", final_balance);
}