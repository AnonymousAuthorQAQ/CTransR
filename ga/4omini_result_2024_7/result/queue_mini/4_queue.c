use std::io::{self, Write};

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Using Vec<i32> for dynamic array
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    fn new(max_elements: usize) -> Result<Self, String> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err("Queue size is too small".to_string());
        }

        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: 0,
            size: 0,
            array: Vec::with_capacity(max_elements),
        })
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = 0;
    }

    fn enqueue(&mut self, x: i32) -> Result<(), String> {
        if self.is_full() {
            return Err("Full queue".to_string());
        }

        self.array.push(x);
        self.rear = (self.rear + 1) % self.capacity;
        self.size += 1;

        Ok(())
    }

    fn front(&self) -> Result<i32, String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }

        Ok(self.array[self.front])
    }

    fn dequeue(&mut self) -> Result<(), String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }

        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;

        Ok(())
    }

    fn front_and_dequeue(&mut self) -> Result<i32, String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }

        let x = self.front()?;
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;

        Ok(x)
    }
}

fn main() {
    let mut queue = Queue::new(12).expect("Failed to create queue");

    for i in 0..10 {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    while !queue.is_empty() {
        let value = queue.front().expect("Failed to access front");
        println!("{}", value);
        queue.dequeue().expect("Failed to dequeue");
    }

    for i in 0..10 {
        queue.enqueue(i).expect("Failed to enqueue");
    }

    while !queue.is_empty() {
        let value = queue.front().expect("Failed to access front");
        println!("{}", value);
        queue.dequeue().expect("Failed to dequeue");
    }
}