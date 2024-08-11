use std::fmt;

pub struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    elements: Vec<i32>,
}

impl Queue {
    pub fn new(max_elements: usize) -> Self {
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

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.size += 1;
        self.rear = (self.rear + 1) % self.capacity;
        self.elements[self.rear] = x;
    }

    pub fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.elements[self.front]
    }

    pub fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
    }

    pub fn front_and_dequeue(&mut self) -> i32 {
        let x = self.front();
        self.dequeue();
        x
    }
}

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut queue_str = String::new();
        let mut current = self.front;
        for _ in 0..self.size {
            queue_str.push_str(&format!("{} ", current));
            current = (current + 1) % self.capacity;
        }
        write!(f, "{}", queue_str.trim())
    }
}

fn main() {
    let mut q = Queue::new(12);

    for i in 0..10 {
        q.enqueue(i);
    }

    println!("Queue: {}", q);
    while !q.is_empty() {
        println!("{}", q.front_and_dequeue());
    }

    for i in 0..10 {
        q.enqueue(i);
    }

    println!("Queue: {}", q);
    while !q.is_empty() {
        println!("{}", q.front_and_dequeue());
    }
}