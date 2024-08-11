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
    const MIN_QUEUE_SIZE: usize = 5;

    pub fn new(max_elements: usize) -> Result<Self, String> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err("Queue size is too small".to_string());
        }
        
        Ok(Self {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
            size: 0,
            array: vec![0; max_elements],
        })
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn enqueue(&mut self, x: i32) -> Result<(), String> {
        if self.is_full() {
            return Err("Full queue".to_string());
        }
        self.rear = (self.rear + 1) % self.capacity; // Circular increment
        self.array[self.rear] = x;
        self.size += 1;
        Ok(())
    }

    pub fn front(&self) -> Result<i32, String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }
        Ok(self.array[self.front])
    }

    pub fn dequeue(&mut self) -> Result<(), String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }
        self.front = (self.front + 1) % self.capacity; // Circular increment
        self.size -= 1;
        Ok(())
    }

    pub fn front_and_dequeue(&mut self) -> Result<i32, String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }
        let x = self.array[self.front];
        self.front = (self.front + 1) % self.capacity; // Circular increment
        self.size -= 1;
        Ok(x)
    }
}

fn main() -> Result<(), String> {
    let mut q = Queue::new(12)?;

    // First enqueue 0 to 9
    for i in 0..10 {
        q.enqueue(i)?;
    }

    // Dequeue and print elements
    while !q.is_empty() {
        println!("{}", q.front()?);
        q.dequeue()?;
    }

    // Enqueue 0 to 9 again
    for i in 0..10 {
        q.enqueue(i)?;
    }

    // Dequeue and print elements
    while !q.is_empty() {
        println!("{}", q.front()?);
        q.dequeue()?;
    }

    Ok(())
}