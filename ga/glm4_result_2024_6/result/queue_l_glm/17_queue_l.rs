use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io::{self, Read};
use std::fmt::Error;

struct Queue {
    capacity: usize,
    elements: VecDeque<i32>,
}

impl Queue {
    fn new(capacity: usize) -> Self {
        if capacity < 5 {
            panic!("Queue size is too small");
        }
        Queue {
            capacity,
            elements: VecDeque::with_capacity(capacity),
        }
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn is_full(&self) -> bool {
        self.elements.len() == self.capacity
    }

    fn enqueue(&mut self, value: i32) {
        if self.is_full() {
            panic!("Queue is full");
        }
        self.elements.push_back(value);
    }

    fn dequeue(&mut self) -> Option<i32> {
        self.elements.pop_front()
    }

    fn front(&self) -> Option<i32> {
        self.elements.front().copied()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let queue_length = input.trim().parse::<usize>().expect("Invalid queue length");

    if queue_length < 5 {
        println!("Queue size is too small");
        return;
    }

    let mut queue = Queue::new(queue_length);

    // Test: Attempt to dequeue from an empty queue
    if let Some(value) = queue.dequeue() {
        println!("Dequeued: {}", value);
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        queue.enqueue(i);
    }

    // Test: Attempt to enqueue to a full queue
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        println!("Please enter a value to enqueue:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let enqueue_value: i32 = input.trim().parse().expect("Invalid input");
        queue.enqueue(enqueue_value);
    }

    // Use FrontAndDequeue to get and remove elements
    while let Some(value) = queue.dequeue() {
        println!("FrontAndDequeue: {}", value);
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i);
    }

    // Use Front and Dequeue
    while let Some(value) = queue.dequeue() {
        println!("Front: {}", value);
    }
}