use std::fmt;
use std::io::{self, Read};

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
    pub fn new(capacity: usize) -> Self {
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

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn enqueue(&mut self, element: i32) {
        if self.is_full() {
            panic!("Queue is full");
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity;
            self.array[self.rear] = element;
        }
    }

    pub fn front(&self) -> i32 {
        if !self.is_empty() {
            self.array[self.front]
        } else {
            panic!("Queue is empty");
        }
    }

    pub fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Queue is empty");
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
        }
    }

    pub fn front_and_dequeue(&mut self) -> i32 {
        let element = self.front();
        self.dequeue();
        element
    }
}

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Queue {{ size: {}, capacity: {} }}", self.size, self.capacity)
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter queue length (at least {}):", MIN_QUEUE_SIZE);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let queue_length: usize = input.trim().parse().expect("Invalid input or queue size too small");

    let mut queue = Queue::new(queue_length);

    // Test: Attempt to dequeue from an empty queue
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        queue.dequeue();
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i as i32);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        println!("Enter a number to enqueue: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let enqueue_value: i32 = input.trim().parse().expect("Invalid input");
        queue.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use front_and_dequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue());
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i as i32);
    }

    // Use front and dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front());
        queue.dequeue();
    }

    println!("Disposed of queue.");
}