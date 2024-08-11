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
    const MIN_QUEUE_SIZE: usize = 5;

    fn new(max_elements: usize) -> Queue {
        if max_elements < Self::MIN_QUEUE_SIZE {
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
        self.front = 1; // For convenience of returning indices, begins with 1
        self.rear = 0;
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0 // Wrap around
        } else {
            value + 1
        }
    }

    fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.size += 1;
        self.rear = self.succ(self.rear);
        self.array[self.rear] = x;
    }

    fn front(&self) -> i32 {
        if !self.is_empty() {
            self.array[self.front]
        } else {
            panic!("Empty queue");
        }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.size -= 1;
        self.front = self.succ(self.front);
    }

    fn front_and_dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        }
        let x = self.front();
        self.size -= 1;
        self.front = self.succ(self.front);
        x
    }
}

fn main() {
    let mut queue = Queue::new(12);

    // Enqueue integers 0 to 9
    for i in 0..10 {
        queue.enqueue(i);
    }

    // Dequeue and print values
    while !queue.is_empty() {
        println!("{}", queue.front());
        queue.dequeue();
    }

    // Enqueue integers 0 to 9 again
    for i in 0..10 {
        queue.enqueue(i);
    }

    // Dequeue and print values again
    while !queue.is_empty() {
        println!("{}", queue.front());
        queue.dequeue();
    }

    // Queue will be automatically dropped at the end of main
}