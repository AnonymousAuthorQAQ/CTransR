use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let m = Arc::new(Mutex::new(()));
    let m1 = m.clone();
    let m2 = m.clone();

    let handle1 = thread::spawn(move || {
        let mut loop_count = 0;
        while loop_count < 2 {
            let _lock = m1.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    let handle2 = thread::spawn(move || {
        let mut loop_count = 0;
        while loop_count < 2 {
            let _lock = m2.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("The thread is over, process is over too.");
}
