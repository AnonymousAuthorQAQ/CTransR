use std::ptr;
use std::process;

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
        if max_elements < 5 {
            eprintln!("Queue size is too small");
            process::exit(1);
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
            process::exit(1);
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
            self.array[self.rear] = x;
        }
    }

    fn front(&self) -> ElementType {
        if self.is_empty() {
            eprintln!("Empty queue");
            process::exit(1);
        }
        self.array[self.front]
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            eprintln!("Empty queue");
            process::exit(1);
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        if self.is_empty() {
            eprintln!("Empty queue");
            process::exit(1);
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

fn main() {
    let mut queue = Queue::create_queue(12);

    for i in 0..10 {
        queue.enqueue(i);
    }

    while !queue.is_empty() {
        println!("{}", queue.front());
        queue.dequeue();
    }

    for i in 0..10 {
        queue.enqueue(i);
    }

    while !queue.is_empty() {
        println!("{}", queue.front());
        queue.dequeue();
    }
}