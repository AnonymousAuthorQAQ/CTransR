use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    let mut loop_count = Arc::new(Mutex::new(0));
    let hello_world = Arc::new(Mutex::new("Hello world"));
    let good_morning = Arc::new(Mutex::new("Good morning"));

    let handles: Vec<thread::JoinHandle<()>> = vec![
        thread::spawn({
            let loop_count = Arc::clone(&loop_count);
            let hello_world = Arc::clone(&hello_world);
            move || {
                for _ in 0..2 {
                    let mut count = loop_count.lock().unwrap();
                    for _ in 0..10 {
                        println!("{}", hello_world.borrow());
                    }
                    *count += 1;
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }),
        thread::spawn({
            let loop_count = Arc::clone(&loop_count);
            let good_morning = Arc::clone(&good_morning);
            move || {
                thread::sleep(Duration::from_secs(1)); // Delay for 1 second
                for _ in 0..2 {
                    let mut count = loop_count.lock().unwrap();
                    for _ in 0..10 {
                        println!("{}", good_morning.borrow());
                    }
                    *count += 1;
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }),
    ];

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The threads are over, process is over too.");
}