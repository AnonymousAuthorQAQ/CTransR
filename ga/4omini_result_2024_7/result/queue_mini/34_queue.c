use std::io;
use std::ptr;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Using Vec<T> for dynamic array
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    fn new(max_elements: usize) -> Result<Queue, &'static str> {
        if max_elements < Queue::MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: 0,
            size: 0,
            array: vec![0; max_elements], // Initialize with zeros
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

    fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        }

        self.size += 1;
        self.array[self.rear] = x;
        self.rear = (self.rear + 1) % self.capacity;
        Ok(())
    }

    fn front(&self) -> Result<i32, &'static str> {
        if self.is_empty() {
            Err("Empty queue")
        } else {
            Ok(self.array[self.front])
        }
    }

    fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        }

        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
        Ok(())
    }

    fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        let value = self.front()?;
        self.dequeue()?;
        Ok(value)
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