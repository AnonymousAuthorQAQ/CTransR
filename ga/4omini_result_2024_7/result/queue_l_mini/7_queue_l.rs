use std::io::{self, Write};

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    fn new(max_elements: usize) -> Queue {
        if max_elements < 5 {
            panic!("Queue size is too small.");
        }
        Queue {
            capacity: max_elements,
            front: 1,
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

    fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue.");
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity; // Wrap around
            self.array[self.rear] = x;
        }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue.");
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity; // Wrap around
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue.");
        }
        self.array[self.front]
    }

    fn front_and_dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue.");
        }
        let x = self.array[self.front];
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity; // Wrap around
        x
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter the queue length (minimum 5): ");
    io::stdout().flush().unwrap(); // Flush to ensure prompt is displayed
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    
    let queue_length: usize = input.trim().parse().expect("Invalid input");

    if queue_length < 5 {
        eprintln!("Invalid input or queue size is too small");
        return;
    }

    let mut queue = Queue::new(queue_length);

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.dequeue();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..(queue_length - 1) {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i as i32);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        print!("Enter a value to enqueue: ");
        io::stdout().flush().unwrap(); // Flush to ensure prompt is displayed
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        
        if let Ok(enqueue_value) = input.trim().parse::<i32>() {
            queue.enqueue(enqueue_value);
        } else {
            eprintln!("Invalid input");
            return;
        }
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue());
    }

    // Enqueue elements again
    for i in 0..(queue_length - 2) {
        queue.enqueue(i as i32);
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front());
        queue.dequeue();
    }
}