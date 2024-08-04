use std::io::{self, Write};

#[derive(Debug)]
pub struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    // Create a new queue with the specified maximum capacity
    pub fn new(max_elements: usize) -> Queue {
        if max_elements < 5 {
            panic!("Queue size is too small");
        }
        Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
            size: 0,
            array: vec![0; max_elements],
        }
    }

    // Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // Check if the queue is full
    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    // Enqueue an element to the queue
    pub fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.rear = (self.rear + 1) % self.capacity;
            self.array[self.rear] = x;
            self.size += 1;
        }
    }

    // Get the front element without dequeuing
    pub fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.array[self.front]
    }

    // Dequeue an element from the queue
    pub fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.front = (self.front + 1) % self.capacity;
            self.size -= 1;
        }
    }

    // Get the front element and dequeue it
    pub fn front_and_dequeue(&mut self) -> i32 {
        let x = self.front();
        self.dequeue();
        x
    }
}

fn main() {
    let mut q = Queue::new(12);

    // Enqueue integers 0 to 9
    for i in 0..10 {
        q.enqueue(i);
    }

    // Print and dequeue until empty
    while !q.is_empty() {
        println!("{}", q.front());
        q.dequeue();
    }

    // Enqueue integers 0 to 9 again
    for i in 0..10 {
        q.enqueue(i);
    }

    // Print and dequeue until empty
    while !q.is_empty() {
        println!("{}", q.front());
        q.dequeue();
    }
}