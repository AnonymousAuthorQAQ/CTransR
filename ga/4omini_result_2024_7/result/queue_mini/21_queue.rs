use std::io::{self, Write};
use std::ptr;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Change type as needed
}

impl Queue {
    fn new(max_elements: usize) -> Queue {
        if max_elements < 5 {
            panic!("Queue size is too small");
        }
        Queue {
            capacity: max_elements,
            front: 1,
            rear: 0,
            size: 0,
            array: vec![0; max_elements],
        }
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

    fn succ(&self, value: usize) -> usize {
        let mut value = value + 1;
        if value == self.capacity {
            value = 0;
        }
        value
    }

    fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
            self.array[self.rear] = x;
        }
    }

    fn front(&self) -> i32 {
        if !self.is_empty() {
            return self.array[self.front];
        }
        panic!("Empty queue");
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
        }
    }

    fn front_and_dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            let x = self.array[self.front];
            self.size -= 1;
            self.front = self.succ(self.front);
            x
        }
    }
}

fn main() {
    let mut queue = Queue::new(12);

    for i in 0..10 {
        queue.enqueue(i);
    }

    while !queue.is_empty() {
        println!("{}", queue.front());
        queue.dequeue();
    }

    for i in 0..10 {
        queue.enqueue(i);
    }

    while !queue.is_empty() {
        println!("{}", queue.front());
        queue.dequeue();
    }
}