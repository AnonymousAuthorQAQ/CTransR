use std::io;
use std::error::Error;

const MIN_QUEUE_SIZE: usize = 5;

type ElementType = i32;

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
            capacity: capacity,
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

    fn enqueue(&mut self, value: ElementType) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.size += 1;
        self.array[self.rear] = value;
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
    let mut queue: Queue;
    let mut queue_length: usize;

    loop {
        println!("Enter the queue length (must be at least {}):", MIN_QUEUE_SIZE);
        match read_input() {
            Ok(length) => {
                queue_length = length;
                if queue_length < MIN_QUEUE_SIZE {
                    println!("Queue size is too small, please try again.");
                } else {
                    break;
                }
            }
            Err(e) => {
                println!("Invalid input: {}", e);
            }
        }
    }

    queue = Queue::new(queue_length);

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.dequeue();
    } else {
        println!("Queue is empty, cannot dequeue.");
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
    if !queue.is_full() {
        println!("Enter a value to enqueue:");
        match read_input() {
            Ok(value) => {
                queue.enqueue(value);
            }
            Err(e) => {
                println!("Invalid input: {}", e);
            }
        }
    } else {
        println!("Queue is full, cannot enqueue.");
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
}

fn read_input() -> Result<usize, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.trim().parse::<usize>()
}