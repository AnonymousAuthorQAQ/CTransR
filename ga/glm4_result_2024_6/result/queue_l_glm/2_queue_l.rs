use std::io;
use std::error::Error;

type ElementType = i32;

const MIN_QUEUE_SIZE: usize = 5;

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

    fn enqueue(&mut self, x: ElementType) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.size += 1;
        self.array[self.rear] = x;
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

fn main() {
    let mut queue_length = 0;
    loop {
        println!("Enter the queue length (>= {}):", MIN_QUEUE_SIZE);
        match io::stdin().read_line(&mut String::new()) {
            Ok(_) => {
                if let Ok(length) = io::stdin().bytes().last() {
                    if let Ok(length) = length {
                        queue_length = length as usize;
                        if queue_length >= MIN_QUEUE_SIZE {
                            break;
                        } else {
                            println!("Queue size is too small, please try again.");
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error reading input: {}", e.description());
                continue;
            }
        }
    }

    let mut q = Queue::new(queue_length);

    // Enqueue elements
    println!("Enter elements to enqueue:");
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input, please try again.");
            continue;
        }
        let input: Vec<&str> = input.trim().split_whitespace().collect();
        for s in input {
            if let Ok(value) = s.parse::<i32>() {
                if q.is_full() {
                    println!("Queue is full, cannot enqueue more elements.");
                    break;
                }
                q.enqueue(value);
            } else {
                println!("Invalid input, please enter integers only.");
            }
        }
        println!("Enter more elements or type 'done' to finish:");
        if input.last().unwrap() == "done" {
            break;
        }
    }

    // Dequeue elements
    while !q.is_empty() {
        println!("Dequeue: {}", q.front_and_dequeue());
    }
}