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
            panic!("Queue size is too small");
        }
        Queue {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
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
            panic!("Full queue");
        }
        self.rear = (self.rear + 1) % self.capacity;
        self.array[self.rear] = x;
        self.size += 1;
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
        }
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
    }

    fn front_and_dequeue(&mut self) -> i32 {
        let x = self.front();
        self.dequeue();
        x
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = self.capacity - 1;
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter queue length: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
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
            queue.enqueue(i as i32);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        input.clear();
        print!("Enter value to enqueue: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let enqueue_value: i32 = input.trim().parse().expect("Invalid input");
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