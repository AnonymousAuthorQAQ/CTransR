use std::mem;
use std::ptr;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    fn new(max_elements: usize) -> Option<Self> {
        if max_elements < Self::MIN_QUEUE_SIZE {
            eprintln!("Queue size is too small");
            return None;
        }

        let array = vec![0; max_elements];
        Some(Self {
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

    fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity;
            self.array[self.rear] = x;
            Ok(())
        }
    }

    fn front(&self) -> Result<i32, &'static str> {
        if !self.is_empty() {
            Ok(self.array[self.front])
        } else {
            Err("Empty queue")
        }
    }

    fn dequeue(&mut self) -> Result<(), &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        } else {
            let x = self.array[self.front];
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
            Ok(x)
        }
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