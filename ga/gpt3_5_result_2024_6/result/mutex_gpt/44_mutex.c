use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn thread1_function(mutex: Arc<Mutex<()>>) {
    let mut loop_count = 0;

    while loop_count < 2 {
        let _lock = mutex.lock().unwrap();

        for _ in 0..10 {
            println!("Hello world");
        }

        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}

fn thread2_function(mutex: Arc<Mutex<()>>) {
    let mut loop_count = 0;
    thread::sleep(Duration::from_secs(1));

    while loop_count < 2 {
        let _lock = mutex.lock().unwrap();

        for _ in 0..10 {
            println!("Good morning");
        }

        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}

fn main() {
    let mutex = Arc::new(Mutex::new(()));
    let mutex_clone = Arc::clone(&mutex);

    let handle1 = thread::spawn(move || {
        thread1_function(mutex_clone);
    });

    let handle2 = thread::spawn(move || {
        thread2_function(mutex);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("The thread is over, process is over too.");
}