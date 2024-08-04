use std::fmt;

#[derive(Debug)]
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

    fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Queue is full");
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity;
            self.array[self.rear] = x;
        }
    }

    fn front(&self) -> i32 {
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
}

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        s.push_str("Queue: ");
        for i in 0..self.size {
            s.push_str(&format!("{}, ", self.array[(self.front + i) % self.capacity]));
        }
        write!(f, "{}", s)
    }
}

fn main() {
    let mut q = Queue::new(12);
    for i in 0..10 {
        q.enqueue(i);
    }
    println!("{}", q);
    while !q.is_empty() {
        println!("{}", q.front());
        q.dequeue();
    }
    for i in 0..10 {
        q.enqueue(i);
    }
    println!("{}", q);
    while !q.is_empty() {
        println!("{}", q.front());
        q.dequeue();
    }
}