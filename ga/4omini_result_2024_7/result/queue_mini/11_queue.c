use std::io::{self, Write};

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Using Vec for dynamic array
}

impl Queue {
    fn new(max_elements: usize) -> Result<Queue, String> {
        if max_elements < 5 {
            return Err("Queue size is too small".to_string());
        }
        Ok(Queue {
            capacity: max_elements,
            front: 1, // We'll use 1-based indexing for consistency with the C code
            rear: 0,
            size: 0,
            array: vec![0; max_elements],
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
        self.front = 1;
        self.rear = 0;
    }

    fn enqueue(&mut self, x: i32) -> Result<(), String> {
        if self.is_full() {
            return Err("Full queue".to_string());
        }
        self.size += 1;
        self.rear = self.succ(self.rear);
        self.array[self.rear] = x;
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
        self.size -= 1;
        self.front = self.succ(self.front);
        Ok(())
    }

    fn front_and_dequeue(&mut self) -> Result<i32, String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }
        let x = self.array[self.front];
        self.size -= 1;
        self.front = self.succ(self.front);
        Ok(x)
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }
}

fn main() {
    let mut q = Queue::new(12).expect("Failed to create queue");

    for i in 0..10 {
        q.enqueue(i).expect("Failed to enqueue");
    }

    while !q.is_empty() {
        let front_value = q.front().expect("Failed to get front");
        println!("{}", front_value);
        q.dequeue().expect("Failed to dequeue");
    }

    for i in 0..10 {
        q.enqueue(i).expect("Failed to enqueue");
    }

    while !q.is_empty() {
        let front_value = q.front().expect("Failed to get front");
        println!("{}", front_value);
        q.dequeue().expect("Failed to dequeue");
    }
}