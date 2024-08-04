use std::fmt;
use std::mem;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    fn new(max_elements: usize) -> Self {
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

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn enqueue(&mut self, element: i32) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity;
            self.array[self.rear] = element;
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.array[self.front]
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
        }
    }

    fn front_and_dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        }
        let value = self.array[self.front];
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
        value
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        mem::drop(self.array);
    }
}

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let mut current = self.front;
        for _ in 0..self.size {
            s.push_str(&self.array[current].to_string());
            current = (current + 1) % self.capacity;
            if current != self.front {
                s.push(' ');
            }
        }
        write!(f, "{}", s)
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