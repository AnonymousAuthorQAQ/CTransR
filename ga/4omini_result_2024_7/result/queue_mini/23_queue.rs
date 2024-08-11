use std::io::{self, Write};
use std::ptr;

#[derive(Debug)]
pub struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    pub fn new(max_elements: usize) -> Result<Queue, &'static str> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        let array = vec![0; max_elements]; // Pre-allocate space for elements
        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
            size: 0,
            array,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        }

        self.size += 1;
        self.rear = (self.rear + 1) % self.capacity;
        self.array[self.rear] = x;

        Ok(())
    }

    pub fn front(&self) -> Result<i32, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        Ok(self.array[self.front])
    }

    pub fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;

        Ok(())
    }

    pub fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }
        let x = self.array[self.front];
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;

        Ok(x)
    }
}

fn main() {
    let mut queue = Queue::new(12).expect("Failed to create queue");

    for i in 0..10 {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    while !queue.is_empty() {
        let value = queue.front().expect("Failed to get front");
        println!("{}", value);
        queue.dequeue().expect("Failed to dequeue");
    }

    for i in 0..10 {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    while !queue.is_empty() {
        let value = queue.front().expect("Failed to get front");
        println!("{}", value);
        queue.dequeue().expect("Failed to dequeue");
    }
}