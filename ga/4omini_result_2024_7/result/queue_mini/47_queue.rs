use std::io::{self, Write};

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    fn new(max_elements: usize) -> Queue {
        if max_elements < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }
        Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements.wrapping_sub(1),
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

    fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.rear = (self.rear + 1) % self.capacity;
        self.array[self.rear] = x;
        self.size += 1;
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.array[self.front]
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
    }

    fn front_and_dequeue(&mut self) -> i32 {
        let x = self.front();
        self.dequeue();
        x
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = self.capacity.wrapping_sub(1);
    }
}

fn main() {
    let mut queue = Queue::new(12);

    // First enqueue phase
    for i in 0..10 {
        queue.enqueue(i);
    }

    // First dequeue phase
    while !queue.is_empty() {
        let value = queue.front();
        println!("{}", value);
        queue.dequeue();
    }

    // Second enqueue phase
    for i in 0..10 {
        queue.enqueue(i);
    }

    // Second dequeue phase
    while !queue.is_empty() {
        let value = queue.front();
        println!("{}", value);
        queue.dequeue();
    }
}