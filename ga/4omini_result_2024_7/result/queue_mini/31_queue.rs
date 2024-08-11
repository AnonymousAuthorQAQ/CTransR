use std::io::{self, Write};

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Using i32 for ElementType
}

impl Queue {
    fn new(max_elements: usize) -> Result<Self, &'static str> {
        const MIN_QUEUE_SIZE: usize = 5;

        if max_elements < MIN_QUEUE_SIZE {
            return Err("Queue size is too small");
        }

        let array = vec![0; max_elements];
        Ok(Queue {
            capacity: max_elements,
            front: 1,
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

    fn enqueue(&mut self, x: i32) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Full queue");
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
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
            self.front = self.succ(self.front);
            Ok(())
        }
    }

    fn front_and_dequeue(&mut self) -> Result<i32, &'static str> {
        if self.is_empty() {
            return Err("Empty queue");
        } else {
            let x = self.array[self.front];
            self.size -= 1;
            self.front = self.succ(self.front);
            Ok(x)
        }
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }

    fn dispose(self) {
        // In Rust, we don't need to manually free memory; it will be cleaned up when Queue goes out of scope.
    }
}

fn main() {
    let mut q = Queue::new(12).expect("Failed to create queue");

    for i in 0..10 {
        q.enqueue(i).expect("Failed to enqueue");
    }

    while !q.is_empty() {
        print!("{}\n", q.front().expect("Failed to get front"));
        q.dequeue().expect("Failed to dequeue");
    }

    for i in 0..10 {
        q.enqueue(i).expect("Failed to enqueue");
    }

    while !q.is_empty() {
        print!("{}\n", q.front().expect("Failed to get front"));
        q.dequeue().expect("Failed to dequeue");
    }

    q.dispose(); // This is optional, as Rust will clean up when q goes out of scope
}