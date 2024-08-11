use std::fmt;
use std::io::{self, Read};

type ElementType = i32;

const MIN_QUEUE_SIZE: usize = 5;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec ElementType,
}

impl Queue {
    fn new(capacity: usize) -> Queue {
        if capacity < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }
        Queue {
            capacity,
            front: 0,
            rear: 0,
            size: 0,
            array: vec![0; capacity],
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn enqueue(&mut self, element: ElementType) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.size += 1;
        self.array[self.rear] = element;
        self.rear = (self.rear + 1) % self.capacity;
    }

    fn front(&self) -> ElementType {
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

    fn front_and_dequeue(&mut self) -> ElementType {
        let value = self.front();
        self.dequeue();
        value
    }
}

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut queue_str = String::new();
        let mut current = self.front;
        for _ in 0..self.size {
            queue_str.push_str(&format!("{}, ", self.array[current]));
            current = (current + 1) % self.capacity;
        }
        write!(f, "[{}]", queue_str.trim_end_matches(", "))
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter the queue size: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let queue_size: usize = input.trim().parse().expect("Invalid input or queue size is too small");

    let mut queue = Queue::new(queue_size);

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.dequeue();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_size - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        println!("Enter a value to enqueue: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let enqueue_value: ElementType = input.trim().parse().expect("Invalid input");
        queue.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue());
    }

    // Enqueue elements again
    for i in 0..queue_size - 2 {
        queue.enqueue(i);
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front());
        queue.dequeue();
    }

    println!("Final Queue: {}", queue);
}