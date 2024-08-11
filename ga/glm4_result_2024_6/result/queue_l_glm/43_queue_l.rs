use std::io::{self, Read};
use std::fmt;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    fn new(capacity: usize) -> Self {
        if capacity < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }
        Queue {
            capacity,
            front: 0,
            rear: 0,
            size: 0,
            array: vec![0; capacity],
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn enqueue(&mut self, value: i32) {
        if self.is_full() {
            panic!("Queue is full");
        }
        self.size += 1;
        self.rear = (self.rear + 1) % self.capacity;
        self.array[self.rear] = value;
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Queue is empty");
        }
        self.array[self.front]
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Queue is empty");
        }
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
    }

    fn front_and_dequeue(&mut self) -> i32 {
        let value = self.front();
        self.dequeue();
        value
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let queue_length: usize = input.trim().parse().expect("Invalid input or queue size is too small");

    let mut q = Queue::new(queue_length);

    // Test: Attempt to dequeue from an empty queue
    if !q.is_empty() {
        q.dequeue();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if q.is_full() {
            println!("Queue is full");
        } else {
            q.enqueue(i as i32);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !q.is_full() {
        println!("Please enter a value to enqueue: ");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let enqueue_value: i32 = input.trim().parse().expect("Invalid input");
        q.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !q.is_empty() {
        println!("FrontAndDequeue: {}", q.front_and_dequeue());
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        q.enqueue(i as i32);
    }

    // Use Front and Dequeue
    while !q.is_empty() {
        println!("Front: {}", q.front());
        q.dequeue();
    }
}