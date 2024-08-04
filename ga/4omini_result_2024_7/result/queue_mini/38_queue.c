use std::io::{self, Write};
use std::ops::{Deref, DerefMut};

type ElementType = i32;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: isize,
    size: usize,
    array: Vec<ElementType>,
}

impl Queue {
    fn new(max_elements: usize) -> Result<Self, String> {
        if max_elements < 5 {
            return Err("Queue size is too small".to_string());
        }
        let array = vec![0; max_elements];
        Ok(Queue {
            capacity: max_elements,
            front: 0,
            rear: -1,
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
        self.rear = -1;
    }

    fn enqueue(&mut self, x: ElementType) -> Result<(), String> {
        if self.is_full() {
            return Err("Full queue".to_string());
        }
        self.size += 1;
        self.rear = self.successor(self.rear);
        self.array[self.rear as usize] = x;
        Ok(())
    }

    fn front(&self) -> Result<ElementType, String> {
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
        self.front = self.successor(self.front as isize) as usize;
        Ok(())
    }

    fn front_and_dequeue(&mut self) -> Result<ElementType, String> {
        let x = self.front()?;
        self.size -= 1;
        self.front = self.successor(self.front as isize) as usize;
        Ok(x)
    }

    fn successor(&self, value: isize) -> isize {
        if value + 1 == self.capacity as isize {
            0
        } else {
            value + 1
        }
    }
}

fn main() {
    let max_elements = 12;
    let mut q = Queue::new(max_elements).expect("Failed to create queue.");

    for i in 0..10 {
        q.enqueue(i).expect("Failed to enqueue.");
    }

    while !q.is_empty() {
        println!("{}", q.front().expect("Failed to read front."));
        q.dequeue().expect("Failed to dequeue.");
    }

    for i in 0..10 {
        q.enqueue(i).expect("Failed to enqueue.");
    }

    while !q.is_empty() {
        println!("{}", q.front().expect("Failed to read front."));
        q.dequeue().expect("Failed to dequeue.");
    }
}