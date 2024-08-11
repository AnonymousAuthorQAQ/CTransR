use std::io::{self, Write};

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
        if max_elements < 5 {
            panic!("Queue size is too small");
        }
        Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1, // This ensures the first element is placed correctly
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
        } else {
            self.rear = (self.rear + 1) % self.capacity; // Wrap around
            self.array[self.rear] = x;
            self.size += 1;
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
            self.front = (self.front + 1) % self.capacity; // Wrap around
            self.size -= 1;
        }
    }

    fn front_and_dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            let item = self.front();
            self.front = (self.front + 1) % self.capacity; // Wrap around
            self.size -= 1;
            item
        }
    }
}

fn main() {
    let mut queue = Queue::new(12);

    // Enqueue integers 0 to 9
    for i in 0..10 {
        queue.enqueue(i);
    }

    // Print dequeued elements
    while !queue.is_empty() {
        println!("{}", queue.front());
        queue.dequeue();
    }

    // Refill queue with integers again
    for i in 0..10 {
        queue.enqueue(i);
    }

    // Print dequeued elements
    while !queue.is_empty() {
        println!("{}", queue.front());
        queue.dequeue();
    }
}