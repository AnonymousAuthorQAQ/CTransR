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

    fn enqueue(&mut self, element: i32) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.size += 1;
        self.array[self.rear] = element;
        self.rear = (self.rear + 1) % self.capacity;
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
        }
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
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

fn main() {
    let mut q: Queue;
    let mut input: i32;

    if let Ok(size) = std::io::stdin().read_line() {
        let size: usize = match size.trim().parse() {
            Ok(size) => size,
            Err(e) => {
                eprintln!("Invalid input: {}", e);
                return;
            }
        };
        if size < 5 {
            eprintln!("Queue size is too small");
            return;
        }
        q = Queue::new(size);
    } else {
        eprintln!("Invalid input");
        return;
    }

    if q.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        q.dequeue();
    }

    while !q.is_full() {
        if let Ok(input) = std::io::stdin().read_line() {
            if let Ok(value) = input.trim().parse() {
                q.enqueue(value);
            } else {
                eprintln!("Invalid input");
                return;
            }
        } else {
            eprintln!("Invalid input");
            return;
        }
    }

    if !q.is_full() {
        if let Ok(input) = std::io::stdin().read_line() {
            if let Ok(value) = input.trim().parse() {
                q.enqueue(value);
            } else {
                eprintln!("Invalid input");
                return;
            }
        } else {
            eprintln!("Invalid input");
            return;
        }
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    while !q.is_empty() {
        println!("FrontAndDequeue: {}", q.front_and_dequeue());
    }

    while !q.is_empty() {
        println!("Front: {}", q.front());
        q.dequeue();
    }
}