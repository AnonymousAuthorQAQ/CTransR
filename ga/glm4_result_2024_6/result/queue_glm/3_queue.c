use std::fmt;

pub struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    pub fn new(capacity: usize) -> Queue {
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

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity;
            self.array[self.rear] = x;
        }
    }

    pub fn front(&self) -> i32 {
        if !self.is_empty() {
            self.array[self.front]
        } else {
            panic!("Empty queue");
        }
    }

    pub fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
        }
    }

    pub fn front_and_dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            let front = self.front;
            self.front = (self.front + 1) % self.capacity;
            self.array[front]
        }
    }
}

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut q = self.clone();
        let mut s = String::new();
        while !q.is_empty() {
            s.push_str(&q.front().to_string());
            s.push(' ');
            q.dequeue();
        }
        write!(f, "{}", s)
    }
}

fn main() {
    let mut q = Queue::new(12);
    for i in 0..10 {
        q.enqueue(i);
    }
    println!("Queue: {}", q);
    while !q.is_empty() {
        println!("{}", q.front());
        q.dequeue();
    }
    for i in 0..10 {
        q.enqueue(i);
    }
    println!("Queue: {}", q);
    while !q.is_empty() {
        println!("{}", q.front());
        q.dequeue();
    }
}