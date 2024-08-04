use std::io;
use std::io::{Error, Read};
use std::fmt;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    elements: Vec<i32>,
}

impl Queue {
    fn new(max_elements: usize) -> Queue {
        if max_elements < 5 {
            panic!("Queue size is too small");
        }
        Queue {
            capacity: max_elements,
            front: 0,
            rear: 0,
            size: 0,
            elements: vec![0; max_elements],
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
            panic!("Queue is full");
        }
        self.size += 1;
        self.rear = (self.rear + 1) % self.capacity;
        self.elements[self.rear] = element;
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Queue is empty");
        }
        self.elements[self.front]
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Queue is empty");
        }
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
    }

    fn front_and_dequeue(&mut self) -> i32 {
        let value = self.front();
        self.dequeue();
        value
    }
}

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut queue_str = String::new();
        for i in 0..self.capacity {
            let index = (self.front + i) % self.capacity;
            queue_str.push_str(&format!("{}, ", self.elements[index]));
        }
        queue_str.pop(); // Remove trailing comma
        write!(f, "[{}] (Size: {})", queue_str, self.size)
    }
}

fn main() {
    let mut queue: Queue;
    let mut input = String::new();

    println!("Enter the queue size:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let queue_size: usize = input.trim().parse().expect("Invalid input or queue size is too small");

    queue = Queue::new(queue_size);

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.dequeue();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_size - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i as i32);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        println!("Enter a value to enqueue:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let enqueue_value: i32 = input.trim().parse().expect("Invalid input");
        queue.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue());
    }

    // Enqueue elements again
    for i in 0..queue_size - 2 {
        queue.enqueue(i as i32);
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front());
        queue.dequeue();
    }

    println!("Final Queue: {}", queue);
}