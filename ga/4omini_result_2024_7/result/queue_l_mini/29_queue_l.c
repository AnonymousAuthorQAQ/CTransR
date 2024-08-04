use std::io;

#[derive(Debug)]
pub struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    const MIN_QUEUE_SIZE: usize = 5;

    pub fn new(max_elements: usize) -> Queue {
        if max_elements < Queue::MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }
        Queue {
            capacity: max_elements,
            front: 1,
            rear: 0,
            size: 0,
            array: vec![0; max_elements], // Initialize with 0s
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.rear = self.successor(self.rear);
            self.array[self.rear] = x;
        }
    }

    pub fn front(&self) -> Option<i32> {
        if self.is_empty() {
            panics!("Empty queue");
        }
        Some(self.array[self.front])
    }

    pub fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            self.front = self.successor(self.front);
        }
    }

    pub fn front_and_dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            let value = self.array[self.front];
            self.size -= 1;
            self.front = self.successor(self.front);
            value
        }
    }

    fn successor(&self, index: usize) -> usize {
        if (index + 1) == self.capacity {
            0
        } else {
            index + 1
        }
    }
}

fn main() {
    let queue_length: usize;
    
    let stdin = io::stdin();
    println!("Enter queue length: ");
    let mut input = String::new();
    
    stdin.read_line(&mut input).expect("Failed to read line");
    queue_length = input.trim().parse::<usize>().expect("Invalid input or queue size is too small");

    let mut q = Queue::new(queue_length);

    // Test: Attempt to dequeue from an empty queue
    if !q.is_empty() {
        q.dequeue();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements
    for i in 0..(queue_length - 1) {
        if q.is_full() {
            println!("Queue is full");
        } else {
            q.enqueue(i as i32);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if q.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        // Get a value to enqueue
        println!("Enter value to enqueue: ");
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        let enqueue_value = input.trim().parse::<i32>().expect("Invalid input");
        q.enqueue(enqueue_value);
    }

    // Use front_and_dequeue and print values
    while !q.is_empty() {
        println!("FrontAndDequeue: {}", q.front_and_dequeue());
    }

    // Enqueue elements again
    for i in 0..(queue_length - 2) {
        q.enqueue(i as i32);
    }

    // Use front and dequeue
    while !q.is_empty() {
        println!("Front: {}", q.front().unwrap());
        q.dequeue();
    }
}