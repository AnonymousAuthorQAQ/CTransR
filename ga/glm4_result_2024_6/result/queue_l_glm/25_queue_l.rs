use std::io;
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();
    let mut queue_length;

    println!("Please enter the queue length: ");
    io::stdin().read_line(&mut queue_length).expect("Failed to read line");
    let queue_length: usize = queue_length.trim().parse().expect("Please enter a valid number");

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.pop_front();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.len() == queue_length {
            println!("Queue is full");
        } else {
            queue.push_back(i);
        }
    }

    // Test: Attempt to enqueue to a full queue
    let mut enqueue_value;
    println!("Please enter a value to enqueue: ");
    io::stdin().read_line(&mut enqueue_value).expect("Failed to read line");
    let enqueue_value: i32 = enqueue_value.trim().parse().expect("Please enter a valid number");

    if queue.len() < queue_length {
        queue.push_back(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use pop_front to get and remove elements
    while !queue.is_empty() {
        println!("pop_front: {}", queue.pop_front().unwrap());
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.push_back(i);
    }

    // Use front and pop_front
    while !queue.is_empty() {
        println!("front: {}", queue.front().unwrap());
        queue.pop_front();
    }
}