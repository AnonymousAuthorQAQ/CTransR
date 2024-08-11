use std::fmt;
use std::io;
use std::result;

type Result<T> = result::Result<T, Box<dyn std::error::Error>>;

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
        if capacity < 5 {
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
            panic!("Queue is full");
        }
        self.size += 1;
        self.array[self.rear] = value;
        self.rear = (self.rear + 1) % self.capacity;
    }

    fn dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Queue is empty");
        }
        self.size -= 1;
        let front_value = self.array[self.front];
        self.front = (self.front + 1) % self.capacity;
        front_value
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Queue is empty");
        }
        self.array[self.front]
    }
}

fn main() -> Result<()> {
    let mut queue = Queue::new(10);
    println!("Initial queue: {:?}", queue);

    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    println!("Queue after enqueue: {:?}", queue);

    println!("Front: {}", queue.front());
    println!("Dequeue: {}", queue.dequeue());
    println!("Queue after dequeue: {:?}", queue);

    Ok(())
}