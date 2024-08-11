use std::io::{self, BufRead};

type ElementType = i32;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<ElementType>,
}

impl Queue {
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn create_queue(max_elements: usize) -> Self {
        let capacity = if max_elements < 5 { 5 } else { max_elements };
        Queue {
            capacity,
            front: 1,
            rear: 0,
            size: 0,
            array: vec![0; capacity],
        }
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 1;
        self.rear = 0;
    }

    fn enqueue(&mut self, x: ElementType) {
        if self.is_full() {
            eprintln!("Full queue");
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
            self.array[self.rear] = x;
        }
    }

    fn front(&self) -> ElementType {
        if !self.is_empty() {
            self.array[self.front]
        } else {
            eprintln!("Empty queue");
            0
        }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            eprintln!("Empty queue");
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        if self.is_empty() {
            eprintln!("Empty queue");
            0
        } else {
            self.size -= 1;
            let x = self.array[self.front];
            self.front = self.succ(self.front);
            x
        }
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
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    let queue_length: usize = input.trim().parse().unwrap();

    let mut Q = Queue::create_queue(queue_length);

    // Test: Attempt to dequeue from an empty queue
    if Q.is_empty() {
        eprintln!("Queue is empty, cannot dequeue.");
    } 

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if Q.is_full() {
            println!("Queue is full");
        } else {
            Q.enqueue(i as ElementType);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !Q.is_full() {
        let mut enqueue_value = String::new();
        handle.read_line(&mut enqueue_value).unwrap();
        let enqueue_value: ElementType = enqueue_value.trim().parse().unwrap();
        Q.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !Q.is_empty() {
        println!("FrontAndDequeue: {}", Q.front_and_dequeue());
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        Q.enqueue(i as ElementType);
    }

    // Use Front and Dequeue
    while !Q.is_empty() {
        println!("Front: {}", Q.front());
        Q.dequeue();
    }
}