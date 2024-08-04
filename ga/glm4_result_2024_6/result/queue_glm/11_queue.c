use std::collections::VecDeque;

type ElementType = i32;

const MIN_QUEUE_SIZE: usize = 5;

struct Queue {
    elements: VecDeque<ElementType>,
}

impl Queue {
    fn new(max_elements: usize) -> Queue {
        if max_elements < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }
        Queue {
            elements: VecDeque::with_capacity(max_elements),
        }
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn is_full(&self) -> bool {
        self.elements.len() == self.elements.capacity()
    }

    fn enqueue(&mut self, element: ElementType) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.elements.push_back(element);
    }

    fn front(&self) -> ElementType {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.elements.front().unwrap()
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.elements.pop_front();
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.elements.pop_front().unwrap()
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