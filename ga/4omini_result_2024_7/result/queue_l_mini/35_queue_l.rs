use std::io;
use std::mem;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
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
            array: vec![0; max_elements],
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
        self.rear = 0;
    }

    fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.array[self.rear] = x;
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity;
        }
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
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
        }
    }

    fn front_and_dequeue(&mut self) -> i32 {
        let x = self.front();
        self.dequeue();
        x
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter the maximum length of the queue (>= {}):", Queue::MIN_QUEUE_SIZE);
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let queue_length: usize = match input.trim().parse() {
        Ok(num) if num >= Queue::MIN_QUEUE_SIZE => num,
        _ => {
            eprintln!("Invalid input or queue size is too small");
            return;
        }
    };

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
    if !queue.is_full() {
        println!("Enter a value to enqueue:");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let enqueue_value: i32 = match input.trim().parse() {
            Ok(num) => num,
            _ => {
                eprintln!("Invalid input");
                return;
            }
        };
        
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
        queue.enqueue(i as i32);
    }

    // Use front and dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front());
        queue.dequeue();
    }
}