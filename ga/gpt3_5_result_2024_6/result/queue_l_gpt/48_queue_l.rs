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

    fn create_queue(max_elements: usize) -> Queue {
        let capacity = max_elements.max(5);
        Queue {
            capacity,
            front: 0,
            rear: capacity - 1,
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
            self.rear = (self.rear + 1) % self.capacity;
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
            self.front = (self.front + 1) % self.capacity;
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        if self.is_empty() {
            eprintln!("Empty queue");
            0
        } else {
            self.size -= 1;
            let x = self.array[self.front];
            self.front = (self.front + 1) % self.capacity;
            x
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let queue_length: usize = match lines.next().unwrap().unwrap().trim().parse() {
        Ok(num) if num >= 5 => num,
        _ => {
            eprintln!("Invalid input or queue size is too small");
            std::process::exit(1);
        }
    };

    let mut queue = Queue::create_queue(queue_length);

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
            queue.enqueue(i as ElementType);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        if let Ok(enqueue_value) = lines.next().unwrap().unwrap().trim().parse() {
            queue.enqueue(enqueue_value);
        } else {
            eprintln!("Invalid input");
            std::process::exit(1);
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
        queue.enqueue(i as ElementType);
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front());
        queue.dequeue();
    }
}