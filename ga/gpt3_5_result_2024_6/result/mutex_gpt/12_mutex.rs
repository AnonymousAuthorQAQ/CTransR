use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let mutex = Arc::new(Mutex::new(()));

    let mutex1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        let mut loop_count = 0;

        while loop_count < 2 {
            let _guard = mutex1.lock().unwrap();
            for _i in 0..10 {
                println!("Hello world");
            }
            std::thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    let mutex2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        let mut loop_count = 0;
        std::thread::sleep(Duration::from_secs(1));

        while loop_count < 2 {
            let _guard = mutex2.lock().unwrap();
            for _i in 0..10 {
                println!("Good morning");
            }
            std::thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("The thread is over, process is over too.");
}