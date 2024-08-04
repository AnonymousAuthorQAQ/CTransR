use std::io::{self, Write};

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    // Creates a new queue with the specified maximum capacity
    fn new(max_elements: usize) -> Queue {
        if max_elements < 5 {
            panic!("Queue size is too small");
        }
        Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
            size: 0,
            array: vec![0; max_elements],
        }
    }

    // Checks if the queue is empty
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    // Checks if the queue is full
    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    // Enqueues an element onto the queue
    fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.rear = (self.rear + 1) % self.capacity;
        self.array[self.rear] = x;
        self.size += 1;
    }

    // Returns the front element without dequeuing it
    fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.array[self.front]
    }

    // Dequeues an element from the queue
    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
    }

    // Returns and dequeues the front element
    fn front_and_dequeue(&mut self) -> i32 {
        let x = self.front();
        self.dequeue();
        x
    }
}

fn main() {
    // Create a queue with a capacity of 12
    let mut queue = Queue::new(12);

    // Enqueue integers from 0 to 9
    for i in 0..10 {
        queue.enqueue(i);
    }

    // Dequeue and print elements until the queue is empty
    while !queue.is_empty() {
        println!("{}", queue.front());
        queue.dequeue();
    }

    // Enqueue integers from 0 to 9 again
    for i in 0..10 {
        queue.enqueue(i);
    }

    // Dequeue and print elements until the queue is empty
    while !queue.is_empty() {
        println!("{}", queue.front());
        queue.dequeue();
    }
}