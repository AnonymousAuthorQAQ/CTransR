use std::io;
use std::mem;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Use Vec<i32> for dynamic array allocation
}

impl Queue {
    fn new(max_elements: usize) -> Self {
        if max_elements < 5 {
            panic!("Queue size is too small");
        }

        Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1, // rear is initially at the last index
            size: 0,
            array: Vec::with_capacity(max_elements),
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
        self.front = 0;
        self.rear = self.capacity - 1;
        self.array.clear();
    }

    fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        }

        self.rear = (self.rear + 1) % self.capacity;
        if self.array.len() < self.capacity {
            self.array.push(x); // Push the new item to the array
        } else {
            self.array[self.rear] = x; // Replace the item in the circular buffer
        }
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

        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
    }

    fn front_and_dequeue(&mut self) -> i32 {
        let front_value = self.front();
        self.dequeue();
        front_value
    }

    fn dispose(self) {
        // Memory is automatically cleaned up when `self` goes out of scope
    }
}

fn main() {
    let mut input = String::new();
    
    // Reading queue length from standard input
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let queue_length: usize = input.trim().parse().expect("Invalid input");
    
    let mut queue = Queue::new(queue_length);

    // Test: Attempt to dequeue from an empty queue
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        queue.dequeue();
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i);
        }
    }

    // Test: Attempt to enqueue to a full queue
    input.clear(); // Clear the buffer
    io::stdin().read_line(&mut input).expect("Failed to read input");
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        let enqueue_value: i32 = input.trim().parse().expect("Invalid input");
        queue.enqueue(enqueue_value);
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue());
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i);
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front());
        queue.dequeue();
    }

    // Dispose the queue (optional in Rust)
    queue.dispose();
}