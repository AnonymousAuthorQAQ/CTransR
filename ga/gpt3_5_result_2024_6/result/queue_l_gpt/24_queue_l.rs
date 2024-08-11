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
        let x;
        if self.is_empty() {
            eprintln!("Empty queue");
            x = 0;
        } else {
            self.size -= 1;
            x = self.array[self.front];
            self.front = self.succ(self.front);
        }
        x
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock();
    let mut lines = input.lines();
    
    let queue_length: usize = match lines.next().unwrap().unwrap().trim().parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid input or queue size is too small");
            std::process::exit(1);
        }
    };

    let mut Q = Queue::create_queue(queue_length);

    if Q.is_empty() {
        eprintln!("Queue is empty, cannot dequeue");
    }

    for i in 0..queue_length - 1 {
        if Q.is_full() {
            println!("Queue is full");
        } else {
            Q.enqueue(i as ElementType);
        }
    }

    if !Q.is_full() {
        let enqueue_value: ElementType = match lines.next().unwrap().unwrap().trim().parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Invalid input");
                std::process::exit(1);
            }
        };
        Q.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue");
    }

    while !Q.is_empty() {
        println!("FrontAndDequeue: {}", Q.front_and_dequeue());
    }

    for i in 0..queue_length - 2 {
        Q.enqueue(i as ElementType);
    }

    while !Q.is_empty() {
        println!("Front: {}", Q.front());
        Q.dequeue();
    }
}