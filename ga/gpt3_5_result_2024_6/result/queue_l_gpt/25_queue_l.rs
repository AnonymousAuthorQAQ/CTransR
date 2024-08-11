use std::io::{self, BufRead};

type ElementType = i32;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<ElementType>,
}

impl Queue {
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn create_queue(max_elements: usize) -> Queue {
        if max_elements < MinQueueSize {
            eprintln!("Queue size is too small");
            std::process::exit(1);
        }

        Queue {
            capacity: max_elements,
            front: 1,
            rear: 0,
            size: 0,
            array: vec![0; max_elements],
        }
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 1;
        self.rear = 0;
    }

    fn enqueue(&mut self, x: ElementType) {
        if self.is_full() {
            eprintln!("Full queue");
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
            self.array[self.rear] = x;
        }
    }

    fn front(&self) -> ElementType {
        if !self.is_empty() {
            self.array[self.front]
        } else {
            eprintln!("Empty queue");
            0
        }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            eprintln!("Empty queue");
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        if self.is_empty() {
            eprintln!("Empty queue");
            0
        } else {
            self.size -= 1;
            let x = self.array[self.front];
            self.front = self.succ(self.front);
            x
        }
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }
}

const MIN_QUEUE_SIZE: usize = 5;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines_iter = handle.lines();

    let queue_length: usize = match lines_iter.next().unwrap().unwrap().trim().parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid input or queue size is too small");
            std::process::exit(1);
        }
    };

    let mut q = Queue::create_queue(queue_length);

    if q.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    }

    for i in 0..queue_length - 1 {
        if q.is_full() {
            println!("Queue is full");
        } else {
            q.enqueue(i as ElementType);
        }
    }

    if !q.is_full() {
        let val: ElementType = match lines_iter.next().unwrap().unwrap().trim().parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Invalid input");
                std::process::exit(1);
            }
        };
        q.enqueue(val);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    while !q.is_empty() {
        println!("FrontAndDequeue: {}", q.front_and_dequeue());
    }

    for i in 0..queue_length - 2 {
        q.enqueue(i as ElementType);
    }

    while !q.is_empty() {
        println!("Front: {}", q.front());
        q.dequeue();
    }
}