use std::io::{self, Read};
use std::ptr;

type ElementType = i32;

const MIN_QUEUE_SIZE: usize = 5;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec ElementType,
}

impl Queue {
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn create_queue(max_elements: usize) -> Option<Queue> {
        if max_elements < MIN_QUEUE_SIZE {
            None
        } else {
            let mut array = Vec::with_capacity(max_elements);
            array.resize(max_elements, 0);
            Some(Queue {
                capacity: max_elements,
                front: 0,
                rear: max_elements - 1,
                size: 0,
                array,
            })
        }
    }

    fn dispose_queue(&mut self) {
        self.array.clear();
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = self.capacity - 1;
    }

    fn enqueue(&mut self, x: ElementType) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.rear = (self.rear + 1) % self.capacity;
            self.array[self.rear] = x;
        }
    }

    fn front(&self) -> ElementType {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.array[self.front]
        }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            let x = self.array[self.front];
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
            x
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let queue_length: usize = input.trim().parse().expect("Invalid input or queue size too small");

    let mut q = match Queue::create_queue(queue_length) {
        Some(q) => q,
        None => {
            eprintln!("Queue size is too small");
            return;
        }
    };

    if !q.is_empty() {
        q.dequeue();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    loop {
        print!("Enter a value to enqueue: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut enqueue_value = String::new();
        io::stdin().read_line(&mut enqueue_value).expect("Failed to read line");
        let enqueue_value: i32 = enqueue_value.trim().parse().expect("Invalid input");

        if !q.is_full() {
            q.enqueue(enqueue_value);
        } else {
            println!("Queue is full, cannot enqueue.");
            break;
        }
    }

    while !q.is_empty() {
        println!("FrontAndDequeue: {}", q.front_and_dequeue());
    }

    while !q.is_empty() {
        println!("Front: {}", q.front());
        q.dequeue();
    }

    q.dispose_queue();
}