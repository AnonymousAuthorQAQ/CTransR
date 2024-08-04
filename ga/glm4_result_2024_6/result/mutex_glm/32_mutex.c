use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let print_mutex = Arc::new(Mutex::new(()));
    let print_mutex2 = print_mutex.clone();

    let handle1 = thread::spawn(move || {
        let mut loop_count = 0;
        while loop_count < 2 {
            let _guard = print_mutex.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            drop(_guard);
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    let handle2 = thread::spawn(move || {
        let mut loop_count = 0;
        while loop_count < 2 {
            let _guard = print_mutex2.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            drop(_guard);
            thread::sleep(Duration::from_secs(1));
            loop_count += 1;
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("The thread is over, process is over too.");
}

fn thread1_function(print_mutex: Arc<Mutex<()>>) {
    let mut loop_count = 0;
    while loop_count < 2 {
        let _guard = print_mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Hello world");
        }
        drop(_guard);
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}

fn thread2_function(print_mutex: Arc<Mutex<()>>) {
    let mut loop_count = 0;
    while loop_count < 2 {
        let _guard = print_mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Good morning");
        }
        drop(_guard);
        thread::sleep(Duration::from_secs(1));
        loop_count += 1;
    }
}