use std::cmp;
use std::fmt;
use std::io;
use std::mem;
use std::ptr;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
pub struct Queue {
    capacity: usize,
    size: usize,
    front: usize,
    rear: usize,
    array: Vec<i32>,
}

impl Queue {
    pub fn new(max_elements: usize) -> Self {
        if max_elements < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }

        Queue {
            capacity: max_elements,
            size: 0,
            front: 0,
            rear: 0,
            array: vec![0; max_elements],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn enqueue(&mut self, value: i32) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.size += 1;
        self.array[self.rear] = value;
        self.rear = (self.rear + 1) % self.capacity;
    }

    pub fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.array[self.front]
    }

    pub fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
    }

    pub fn front_and_dequeue(&mut self) -> i32 {
        let value = self.front();
        self.dequeue();
        value
    }
}

fn main() {
    let mut queue = Queue::new(10);

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.dequeue();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..9 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        let enqueue_value = 10;
        queue.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue());
    }

    // Enqueue elements again
    for i in 0..9 {
        queue.enqueue(i);
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front());
        queue.dequeue();
    }
}