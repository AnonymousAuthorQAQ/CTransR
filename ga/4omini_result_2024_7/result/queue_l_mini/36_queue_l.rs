use std::io::{self, Write};

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>, // Using Vec for dynamic array allocation
}

impl Queue {
    fn new(max_elements: usize) -> Queue {
        if max_elements < 5 {
            panic!("Queue size is too small");
        }
        Queue {
            capacity: max_elements,
            front: 1,
            rear: 0,
            size: 0,
            array: vec![0; max_elements], // Initialize with 0
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
        self.front = 1;
        self.rear = 0;
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
        let x = self.array[self.front];
        self.size -= 1;
        self.front = self.succ(self.front);
        x
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter queue length: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    
    let queue_length: usize = input.trim().parse().unwrap_or(0);
    if queue_length < 5 {
        eprintln!("Invalid input or queue size is too small");
        std::process::exit(1);
    }

    let mut queue = Queue::new(queue_length);

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
            queue.enqueue(i as i32);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        print!("Enter value to enqueue: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let enqueue_value: i32 = input.trim().parse().unwrap_or(0);

        queue.enqueue(enqueue_value);
    }

    // Use front_and_dequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue());
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i as i32);
    }

    // Use front and dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front());
        queue.dequeue();
    }
}