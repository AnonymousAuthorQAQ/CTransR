use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let queue_length: usize = match input.trim().parse() {
        Ok(length) if length >= 5 => length,
        _ => {
            eprintln!("Invalid input or queue size is too small");
            return;
        }
    };

    let mut queue = VecDeque::with_capacity(queue_length);

    // Test: Attempt to dequeue from an empty queue
    if let Some(_) = queue.front() {
        queue.pop_front();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_capacity(queue) {
        if queue_capacity(queue) == 0 {
            println!("Queue is full");
        } else {
            queue.push_back(i);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if queue_capacity(queue) > 0 {
        match input.trim().parse::<i32>() {
            Ok(value) => queue.push_back(value),
            Err(_) => {
                eprintln!("Invalid input");
                return;
            }
        }
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while let Some(value) = queue.pop_front() {
        println!("FrontAndDequeue: {}", value);
    }

    // Enqueue elements again
    for i in 0..queue_capacity(queue) {
        queue.push_back(i);
    }

    // Use Front and Dequeue
    while let Some(value) = queue.front() {
        println!("Front: {}", value);
        queue.pop_front();
    }
}

fn queue_capacity(queue: &VecDeque<i32>) -> usize {
    queue.capacity() - queue.len()
}