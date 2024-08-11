use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let mutex = Arc::new(Mutex::new(()));

    let mutex1 = Arc::clone(&mutex);
    let handle1 = thread::spawn(move || {
        let mut loop_count = 0;

        while loop_count < 2 {
            let _lock = mutex1.lock().unwrap();
            for _i in 0..10 {
                println!("Hello world");
            }
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    let mutex2 = Arc::clone(&mutex);
    let handle2 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        let mut loop_count = 0;

        while loop_count < 2 {
            let _lock = mutex2.lock().unwrap();
            for _i in 0..10 {
                println!("Good morning");
            }
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("The threads are over, process is over too.");
}