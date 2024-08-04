use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let mutex = Arc::new(Mutex::new(()));

    let mutex1 = Arc::clone(&mutex);
    let thread1 = thread::spawn(move || {
        thread_function("Hello world", mutex1);
    });

    let mutex2 = Arc::clone(&mutex);
    let thread2 = thread::spawn(move || {
        thread_function("Good morning", mutex2);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("The thread is over, process is over too.");
}

fn thread_function(message: &str, mutex: Arc<Mutex<()>>) {
    let mut loop_count = 0;
    while loop_count < 2 {
        let _guard = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("{}", message);
        }
        drop(_guard);
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}