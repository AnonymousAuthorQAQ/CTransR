use std::io;
use std::io::{self, Read};

type ElementType = i32;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    fn new(capacity: usize) -> Queue {
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

    fn enqueue(&mut self, value: ElementType) {
        if self.is_full() {
            panic!("Queue is full");
        } else {
            self.size += 1;
            self.array[self.rear] = value;
            self.rear = (self.rear + 1) % self.capacity;
        }
    }

    fn front(&self) -> ElementType {
        if self.is_empty() {
            panic!("Queue is empty");
        }
        self.array[self.front]
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Queue is empty");
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        let value = self.front();
        self.dequeue();
        value
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let queue_length: usize = input.trim().parse().expect("Invalid input");

    if queue_length < 5 {
        eprintln!("Queue size is too small");
        return;
    }

    let mut q = Queue::new(queue_length);

    if !q.is_empty() {
        q.dequeue();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    for i in 0..queue_length - 1 {
        if q.is_full() {
            println!("Queue is full");
        } else {
            q.enqueue(i);
        }
    }

    if !q.is_full() {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let enqueue_value: i32 = input.trim().parse().expect("Invalid input");
        q.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    while !q.is_empty() {
        println!("FrontAndDequeue: {}", q.front_and_dequeue());
    }

    for i in 0..queue_length - 2 {
        q.enqueue(i);
    }

    while !q.is_empty() {
        println!("Front: {}", q.front());
        q.dequeue();
    }
}