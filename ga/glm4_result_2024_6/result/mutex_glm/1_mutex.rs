use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn thread1_function(mutex: Arc<Mutex<()>>);
fn thread2_function(mutex: Arc<Mutex<()>>);

fn main() {
    let mutex = Arc::new(Mutex::new(()));
    
    let mutex1 = Arc::clone(&mutex);
    let handle1 = thread::spawn(move || {
        for _ in 0..2 {
            let _guard = mutex1.lock().unwrap();
            for _ in 0..10 {
                println!("Hello world");
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    let mutex2 = Arc::clone(&mutex);
    let handle2 = thread::spawn(move || {
        for _ in 0..2 {
            let _guard = mutex2.lock().unwrap();
            for _ in 0..10 {
                println!("Good morning");
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("The thread is over, process is over too.");
}

fn thread1_function(mutex: Arc<Mutex<()>>) {
    for _ in 0..2 {
        let _guard = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Hello world");
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn thread2_function(mutex: Arc<Mutex<()>>) {
    for _ in 0..2 {
        let _guard = mutex.lock().unwrap();
        for _ in 0..10 {
            println!("Good morning");
        }
        thread::sleep(Duration::from_secs(1));
    }
}