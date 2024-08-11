use std::io;
use std::io::Write;
use std::fmt::Debug;

#[derive(Debug)]
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

    fn front(&self) -> Option<i32> {
        if !self.is_empty() {
            Some(self.array[self.front])
        } else {
            None
        }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.size -= 1;
        self.front = self.succ(self.front);
    }

    fn front_and_dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            let x = self.array[self.front];
            self.size -= 1;
            self.front = self.succ(self.front);
            Some(x)
        }
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }

    fn dispose(self) {
        // Rust automatically handles memory; we can use this if needed
    }
}

fn main() {
    let mut input = String::new();

    print!("Enter queue length: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let queue_length: usize = input.trim().parse().expect("Invalid input or queue size is too small");

    let mut queue = Queue::new(queue_length);

    // Test: Attempt to dequeue from an empty queue
    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i as i32);
        }
    }

    // Test: Attempt to enqueue to a full queue
    input.clear();
    io::stdout().flush().unwrap();
    print!("Enter a value to enqueue: ");
    io::stdin().read_line(&mut input).expect("Invalid input");

    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        let enqueue_value: i32 = input.trim().parse().expect("Invalid input");
        queue.enqueue(enqueue_value);
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {:?}", queue.front_and_dequeue().unwrap());
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i as i32);
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {:?}", queue.front().unwrap());
        queue.dequeue();
    }

    queue.dispose();
}