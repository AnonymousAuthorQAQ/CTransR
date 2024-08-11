use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut queue_length: i32;

    if let Err(e) = std::io::stdin().read_line(&mut String::new()) {
        eprintln!("Error reading input: {}", e);
        std::process::exit(1);
    }

    if let Ok(length_str) = std::io::stdin().read_line(&mut String::new()) {
        if let Ok(length) = length_str.trim().parse::<i32>() {
            queue_length = length;
        } else {
            eprintln!("Invalid input or queue size is too small");
            std::process::exit(1);
        }
    } else {
        eprintln!("Error reading input");
        std::process::exit(1);
    }

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.pop_front();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.push_back(i);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        if let Ok(value_str) = std::io::stdin().read_line(&mut String::new()) {
            if let Ok(value) = value_str.trim().parse::<i32>() {
                queue.push_back(value);
            } else {
                eprintln!("Invalid input");
                std::process::exit(1);
            }
        } else {
            eprintln!("Error reading input");
            std::process::exit(1);
        }
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.pop_front().unwrap());
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.push_back(i);
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue[0]);
        queue.pop_front();
    }
}