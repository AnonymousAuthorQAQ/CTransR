use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let loop_count = Arc::new(Mutex::new(0));

    let loop_count1 = loop_count.clone();
    let handle1 = thread::spawn(move || {
        let mut loop_count = loop_count1.lock().unwrap();
        while *loop_count < 2 {
            println!("Hello world");
            *loop_count += 1;
            thread::sleep(Duration::from_secs(1));
        }
    });

    let loop_count2 = loop_count.clone();
    let handle2 = thread::spawn(move || {
        let mut loop_count = loop_count2.lock().unwrap();
        while *loop_count < 2 {
            println!("Good morning");
            *loop_count += 1;
            thread::sleep(Duration::from_secs(1));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("The thread is over, process is over too.");
}

fn thread1_function() {
    let loop_count = 2;
    for _ in 0..loop_count {
        println!("Hello world");
        thread::sleep(Duration::from_secs(1));
    }
}

fn thread2_function() {
    let loop_count = 2;
    for _ in 0..loop_count {
        println!("Good morning");
        thread::sleep(Duration::from_secs(1));
    }
}