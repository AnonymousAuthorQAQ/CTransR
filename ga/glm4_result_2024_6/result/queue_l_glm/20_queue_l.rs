use std::io;
use std::io::{Error, ErrorKind};

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
        self.array[self.rear] = value;
        self.rear = (self.rear + 1) % self.capacity;
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
}

fn main() {
    let mut queue_size = 0;
    println!("Enter the queue size: ");
    io::stdin()
        .read_line(&mut queue_size)
        .expect("Failed to read line");

    let queue_size: usize = queue_size.trim().parse().expect("Invalid input or queue size is too small");

    let mut queue = Queue::new(queue_size);

    // Test: Attempt to dequeue from an empty queue
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        queue.dequeue();
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_size - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i);
        }
    }

    // Test: Attempt to enqueue to a full queue
    let mut enqueue_value = String::new();
    println!("Enter a value to enqueue: ");
    io::stdin()
        .read_line(&mut enqueue_value)
        .expect("Failed to read line");
    let enqueue_value: i32 = enqueue_value.trim().parse().expect("Invalid input");

    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        queue.enqueue(enqueue_value);
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front());
        queue.dequeue();
    }

    // Enqueue elements again
    for i in 0..queue_size - 2 {
        queue.enqueue(i);
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front());
        queue.dequeue();
    }
}