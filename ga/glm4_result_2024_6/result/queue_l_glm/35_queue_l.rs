use std::fmt;
use std::io::{self, Read};
use std::ptr;

const MIN_QUEUE_SIZE: usize = 5;

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

    fn enqueue(&mut self, value: i32) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.size += 1;
        self.array[self.rear] = value;
        self.rear = (self.rear + 1) % self.capacity;
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            let front_value = self.array[self.front];
            self.front = (self.front + 1) % self.capacity;
            Some(front_value)
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

fn main() {
    let mut queue_length = 0;
    loop {
        println!("Please enter the queue length (at least 5):");
        queue_length = read_input();
        if queue_length < MIN_QUEUE_SIZE {
            println!("Queue size is too small, please try again.");
        } else {
            break;
        }
    }

    let mut q = Queue::new(queue_length);

    // Test: Attempt to dequeue from an empty queue
    if !q.is_empty() {
        println!("Dequeue: {:?}", q.dequeue());
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if q.is_full() {
            println!("Queue is full");
        } else {
            q.enqueue(i);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !q.is_full() {
        println!("Please enter an integer to enqueue:");
        let enqueue_value = read_input();
        q.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use front_and_dequeue to get and remove elements
    while !q.is_empty() {
        println!("FrontAndDequeue: {}", q.dequeue().unwrap());
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        q.enqueue(i);
    }

    // Use front and dequeue
    while !q.is_empty() {
        println!("Front: {}", q.front().unwrap());
        q.dequeue();
    }
}

fn read_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut q = Queue::new(5);
        assert!(q.is_empty());
        assert!(!q.is_full());

        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        q.enqueue(4);
        q.enqueue(5);

        assert!(q.is_full());
        assert_eq!(q.dequeue(), Some(1));
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), Some(3));
        assert_eq!(q.dequeue(), Some(4));
        assert_eq!(q.dequeue(), Some(5));
        assert!(q.is_empty());
    }
}