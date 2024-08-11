use std::fmt;
use std::io::{self, Read};

const MIN_QUEUE_SIZE: usize = 5;

type ElementType = i32;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<ElementType>,
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
            panic!("Queue is full");
        }
        self.size += 1;
        self.rear = (self.rear + 1) % self.capacity;
        self.array[self.rear] = element;
    }

    fn front(&self) -> ElementType {
        if self.is_empty() {
            panic!("Queue is empty");
        }
        self.array[self.front]
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Queue is empty");
        }
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        let front = self.front();
        self.dequeue();
        front
    }
}

fn main() {
    let mut queue_size = 0;
    loop {
        println!("Please enter the queue size (must be at least {}):", MIN_QUEUE_SIZE);
        io::stdin().read_line(&mut queue_size).expect("Failed to read line");
        if let Ok(size) = queue_size.trim().parse::<usize>() {
            if size < MIN_QUEUE_SIZE {
                println!("Queue size is too small, try again.");
            } else {
                break;
            }
        } else {
            println!("Invalid input, try again.");
        }
    }

    let mut q = Queue::new(queue_size);

    // Test: Attempt to dequeue from an empty queue
    if !q.is_empty() {
        q.dequeue();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_size - 1 {
        if q.is_full() {
            println!("Queue is full");
        } else {
            q.enqueue(i);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !q.is_full() {
        println!("Please enter a value to enqueue:");
        let mut value = String::new();
        io::stdin().read_line(&mut value).expect("Failed to read line");
        if let Ok(val) = value.trim().parse::<i32>() {
            q.enqueue(val);
        } else {
            println!("Invalid input");
        }
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !q.is_empty() {
        println!("FrontAndDequeue: {}", q.front_and_dequeue());
    }

    // Enqueue elements again
    for i in 0..queue_size - 2 {
        q.enqueue(i);
    }

    // Use Front and Dequeue
    while !q.is_empty() {
        println!("Front: {}", q.front());
        q.dequeue();
    }
}