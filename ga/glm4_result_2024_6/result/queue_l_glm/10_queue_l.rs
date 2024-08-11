use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<i32> = VecDeque::with_capacity(5);

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.pop_front();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..4 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.push_back(i);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        let enqueue_value = 4;
        queue.push_back(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.pop_front().unwrap());
    }

    // Enqueue elements again
    for i in 0..3 {
        queue.push_back(i);
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue[0]);
        queue.pop_front();
    }
}