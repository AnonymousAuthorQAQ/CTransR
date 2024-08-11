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
    pub fn new(max_elements: usize) -> Queue {
        if max_elements < 5 {
            panic!("Queue size is too small");
        }
        Queue {
            capacity: max_elements,
            front: 0,
            rear: 0,
            size: 0,
            array: vec![0; max_elements],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn enqueue(&mut self, element: i32) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.array[self.rear] = element;
            self.rear = (self.rear + 1) % self.capacity;
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
        if !self.is_empty() {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
        } else {
            panic!("Empty queue");
        }
    }

    pub fn front_and_dequeue(&mut self) -> i32 {
        if !self.is_empty() {
            let front = self.array[self.front];
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
            front
        } else {
            panic!("Empty queue");
        }
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        println!("Dropping Queue");
    }
}

fn main() {
    let mut q = Queue::new(12);

    for i in 0..10 {
        q.enqueue(i);
    }

    while !q.is_empty() {
        println!("{}", q.front());
        q.dequeue();
    }

    for i in 0..10 {
        q.enqueue(i);
    }

    while !q.is_empty() {
        println!("{}", q.front());
        q.dequeue();
    }
}