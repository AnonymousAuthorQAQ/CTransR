use std::fmt;
use std::ops::{Index, IndexMut};
use std::ptr::null;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Box<[i32]>,
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
            array: vec![0; capacity].into_boxed_slice(),
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
        self.array[self.rear] = element;
        self.rear = (self.rear + 1) % self.capacity;
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            let value = self.array[self.front];
            self.front = (self.front + 1) % self.capacity;
            Some(value)
        }
    }

    fn front(&self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            Some(self.array[self.front])
        }
    }
}

impl Index<usize> for Queue {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl IndexMut<usize> for Queue {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        s.push('[');
        for i in 0..self.size {
            s.push_str(&self[(self.front + i) % self.capacity].to_string());
            if i < self.size - 1 {
                s.push(',');
            }
        }
        s.push(']');
        write!(f, "{}", s)
    }
}

fn main() {
    let mut queue_size = 0;
    println!("Enter the queue size: ");
    std::io::stdin().read_line(&mut queue_size).expect("Failed to read line");

    let queue_size: usize = queue_size.trim().parse().expect("Invalid input or queue size is too small");

    let mut queue = Queue::new(queue_size);

    // Test: Attempt to dequeue from an empty queue
    if let Some(value) = queue.dequeue() {
        println!("Dequeued: {}", value);
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_size - 1 {
        queue.enqueue(i);
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        println!("Enter a value to enqueue: ");
        let mut value = String::new();
        std::io::stdin().read_line(&mut value).expect("Failed to read line");
        let value: i32 = value.trim().parse().expect("Invalid input");
        queue.enqueue(value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use front and dequeue to get and remove elements
    while let Some(value) = queue.dequeue() {
        println!("Dequeued: {}", value);
    }

    // Enqueue elements again
    for i in 0..queue_size - 2 {
        queue.enqueue(i);
    }

    // Use front and dequeue
    while let Some(value) = queue.front() {
        println!("Front: {}", value);
        queue.dequeue();
    }

    println!("Final queue: {}", queue);
}