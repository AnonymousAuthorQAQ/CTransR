use std::io::{self, Write};

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
    const MIN_QUEUE_SIZE: usize = 5;

    fn new(max_elements: usize) -> Queue {
        if max_elements < Self::MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }

        Queue {
            capacity: max_elements,
            front: 0,
            rear: 0,
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

    fn enqueue(&mut self, x: ElementType) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.array.push(x);
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity;
        }
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
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            let x = self.array[self.front];
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
            x
        }
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = 0;
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter the maximum queue size:");
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let queue_length: usize = input.trim().parse().expect("Invalid input or queue size is too small");

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
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        let mut enqueue_value = String::new();
        println!("Enter a value to enqueue:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut enqueue_value).expect("Failed to read line");
        let value: ElementType = enqueue_value.trim().parse().expect("Invalid input");
        queue.enqueue(value);
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue());
    }

    // Re-enqueue elements
    for i in 0..queue_length - 2 {
        queue.enqueue(i);
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front());
        queue.dequeue();
    }
}