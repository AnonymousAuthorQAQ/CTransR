use std::io;
use std::cmp::Ordering;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    fn new(capacity: usize) -> Self {
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

    fn enqueue(&mut self, element: i32) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.array[self.rear] = element;
            self.rear = (self.rear + 1) % self.capacity;
        }
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
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
        }
    }

    fn front_and_dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            let result = self.array[self.front];
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
            result
        }
    }
}

fn main() {
    let mut queue_capacity = 0;
    println!("Enter the queue capacity: ");
    io::stdin().read_line(&mut queue_capacity).expect("Failed to read line");
    let queue_capacity: usize = queue_capacity.trim().parse().expect("Please enter a number");

    let mut queue = Queue::new(queue_capacity);

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.dequeue();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_capacity - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        println!("Enter an integer to enqueue: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let enqueue_value: i32 = input.trim().parse().expect("Please enter a valid integer");
        queue.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue());
    }

    // Enqueue elements again
    for i in 0..queue_capacity - 2 {
        queue.enqueue(i);
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front());
        queue.dequeue();
    }

    println!("Queue disposed");
}