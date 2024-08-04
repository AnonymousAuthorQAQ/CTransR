use std::io::{self, Write};

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Using i32 as ElementType
}

impl Queue {
    fn new(max_elements: usize) -> Result<Queue, String> {
        if max_elements < MIN_QUEUE_SIZE {
            return Err("Queue size is too small".to_string());
        }

        let array = vec![0; max_elements]; // Initialize the array with zeros

        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: 0,
            size: 0,
            array,
        })
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn enqueue(&mut self, x: i32) -> Result<(), String> {
        if self.is_full() {
            return Err("Full queue".to_string());
        } else {
            self.size += 1;
            self.array[self.rear] = x;
            self.rear = (self.rear + 1) % self.capacity; // Circular increment
            Ok(())
        }
    }

    fn front(&self) -> Result<i32, String> {
        if self.is_empty() {
            Err("Empty queue".to_string())
        } else {
            Ok(self.array[self.front])
        }
    }

    fn dequeue(&mut self) -> Result<(), String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity; // Circular increment
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<i32, String> {
        let value = self.front()?;
        self.dequeue()?;
        Ok(value)
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = 0;
    }
}

fn main() {
    let mut queue = Queue::new(12).expect("Failed to create queue");

    for i in 0..10 {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    while !queue.is_empty() {
        println!("{}", queue.front().expect("Failed to get front"));
        queue.dequeue().expect("Failed to dequeue");
    }

    for i in 0..10 {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    while !queue.is_empty() {
        println!("{}", queue.front().expect("Failed to get front"));
        queue.dequeue().expect("Failed to dequeue");
    }
}