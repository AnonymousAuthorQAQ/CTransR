use std::collections::VecDeque;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    elements: VecDeque<i32>,
}

impl Queue {
    fn new(capacity: usize) -> Self {
        Queue {
            capacity,
            elements: VecDeque::with_capacity(capacity),
        }
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn is_full(&self) -> bool {
        self.elements.len() == self.capacity
    }

    fn enqueue(&mut self, value: i32) {
        if self.is_full() {
            panic!("Queue is full");
        }
        self.elements.push_back(value);
    }

    fn dequeue(&mut self) -> Option<i32> {
        self.elements.pop_front()
    }

    fn front(&self) -> Option<i32> {
        self.elements.front().copied()
    }

    fn front_and_dequeue(&mut self) -> Option<i32> {
        self.elements.pop_front()
    }
}

fn main() {
    let mut queue = Queue::new(5);
    let mut i = 0;

    // Attempt to dequeue from an empty queue
    if let None = queue.dequeue() {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    while i < 4 {
        queue.enqueue(i);
        i += 1;
    }

    // Test: Attempt to enqueue to a full queue
    if let Err(_) = queue.enqueue(i) {
        println!("Queue is full, cannot enqueue.");
    }

    // Use front_and_dequeue to get and remove elements
    while let Some(value) = queue.front_and_dequeue() {
        println!("FrontAndDequeue: {}", value);
    }

    // Enqueue elements again
    for j in 0..3 {
        queue.enqueue(j);
    }

    // Use Front and Dequeue
    while let Some(value) = queue.dequeue() {
        println!("Front: {}", value);
    }
}