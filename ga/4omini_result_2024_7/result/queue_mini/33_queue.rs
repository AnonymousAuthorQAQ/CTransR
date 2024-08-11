use std::io::{self, Write};
use std::mem;

type ElementType = i32;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<ElementType>,
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    fn new(max_elements: usize) -> Result<Queue, String> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            return Err(String::from("Queue size is too small"));
        }

        let array = vec![0; max_elements]; // Initialize with zeros
        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
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

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = self.capacity - 1;
    }

    fn enqueue(&mut self, x: ElementType) -> Result<(), String> {
        if self.is_full() {
            return Err(String::from("Full queue"));
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
            self.array[self.rear] = x;
            Ok(())
        }
    }

    fn front(&self) -> Result<ElementType, String> {
        if !self.is_empty() {
            Ok(self.array[self.front])
        } else {
            Err(String::from("Empty queue"))
        }
    }

    fn dequeue(&mut self) -> Result<(), String> {
        if self.is_empty() {
            Err(String::from("Empty queue"))
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<ElementType, String> {
        if self.is_empty() {
            Err(String::from("Empty queue"))
        } else {
            let x = self.array[self.front];
            self.size -= 1;
            self.front = self.succ(self.front);
            Ok(x)
        }
    }

    fn succ(&self, value: usize) -> usize {
        (value + 1) % self.capacity
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