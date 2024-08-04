use std::io::{self, BufRead};
use std::ptr;
use std::process::exit;

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
    fn new(max_elements: usize) -> Option<Queue> {
        if max_elements < 5 {
            eprintln!("Queue size is too small");
            return None;
        }

        Some(Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
            size: 0,
            array: vec![0; max_elements],
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
        self.rear = self.capacity - 1;
    }

    fn enqueue(&mut self, x: ElementType) {
        if self.is_full() {
            eprintln!("Full queue");
            exit(1);
        } else {
            self.rear = (self.rear + 1) % self.capacity;
            self.array[self.rear] = x;
            self.size += 1;
        }
    }

    fn front(&self) -> ElementType {
        if self.is_empty() {
            eprintln!("Empty queue");
            exit(1);
        }
        self.array[self.front]
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            eprintln!("Empty queue");
            exit(1);
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        let x = self.front();
        self.dequeue();
        x
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();
    
    handle.read_line(&mut input).expect("Failed to read line");
    let queue_length: usize = input.trim().parse().expect("Invalid input or queue size is too small");

    let mut queue = Queue::new(queue_length).expect("Failed to create queue");

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
            queue.enqueue(i as ElementType);
        }
    }

    println!("Enter value to enqueue:");
    input.clear();
    handle.read_line(&mut input).expect("Failed to read line");

    if !queue.is_full() {
        let enqueue_value: ElementType = input.trim().parse().expect("Invalid input");
        queue.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use front_and_dequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue());
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i as ElementType);
    }

    // Use front and dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front());
        queue.dequeue();
    }
}