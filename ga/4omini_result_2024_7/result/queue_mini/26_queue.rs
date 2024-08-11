use std::io::{self, Write};

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    // Create a new queue with a given capacity
    fn new(max_elements: usize) -> Self {
        assert!(max_elements >= 5, "Queue size is too small");
        Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
            size: 0,
            array: vec![0; max_elements],
        }
    }

    // Check if the queue is empty
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    // Check if the queue is full
    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    // Add an element to the queue
    fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.rear = (self.rear + 1) % self.capacity;
        self.array[self.rear] = x;
        self.size += 1;
    }

    // Return the front item of the queue without removing it
    fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.array[self.front]
    }

    // Remove an element from the queue
    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
    }

    // Return and remove the front item of the queue
    fn front_and_dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        }
        let x = self.front();
        self.dequeue();
        x
    }
}

fn main() {
    let mut queue = Queue::new(12);

    for i in 0..10 {
        queue.enqueue(i);
    }

    while !queue.is_empty() {
        println!("{}", queue.front());
        queue.dequeue();
    }

    for i in 0..10 {
        queue.enqueue(i);
    }

    while !queue.is_empty() {
        println!("{}", queue.front());
        queue.dequeue();
    }
}