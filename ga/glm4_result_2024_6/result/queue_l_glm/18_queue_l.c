use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<i32> = VecDeque::new();

    // Test: Attempt to dequeue from an empty queue
    if let Some(_) = queue.pop_front() {
        println!("Dequeued from an empty queue");
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue.capacity() - 1 {
        queue.push_back(i);
    }

    // Test: Attempt to enqueue to a full queue
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        queue.push_back(queue.capacity());
    }

    // Use FrontAndDequeue to get and remove elements
    while let Some(value) = queue.pop_front() {
        println!("FrontAndDequeue: {}", value);
    }

    // Enqueue elements again
    for i in 0..queue.capacity() - 2 {
        queue.push_back(i);
    }

    // Use Front and Dequeue
    while let Some(value) = queue.pop_front() {
        println!("Front: {}", value);
    }
}