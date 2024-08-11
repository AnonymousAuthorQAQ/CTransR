use std::ptr;

type ElementType = i32;

const MIN_QUEUE_SIZE: usize = 5;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Box<[ElementType]>,
}

impl Queue {
    fn new(max_elements: usize) -> Self {
        if max_elements < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }
        Queue {
            capacity: max_elements,
            front: 0,
            rear: 0,
            size: 0,
            array: vec![0; max_elements].into_boxed_slice(),
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn enqueue(&mut self, x: ElementType) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.size += 1;
        self.array[self.rear] = x;
        self.rear = (self.rear + 1) % self.capacity;
    }

    fn front(&self) -> ElementType {
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

    fn front_and_dequeue(&mut self) -> ElementType {
        let x = self.front();
        self.dequeue();
        x
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