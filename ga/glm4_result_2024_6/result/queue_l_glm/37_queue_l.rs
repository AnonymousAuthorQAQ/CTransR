use std::collections::VecDeque;

#[derive(Debug)]
struct Queue {
    elements: VecDeque<i32>,
}

impl Queue {
    pub fn new(max_elements: usize) -> Self {
        if max_elements < 5 {
            panic!("Queue size is too small");
        }
        Queue {
            elements: VecDeque::with_capacity(max_elements),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.elements.len() == self.elements.capacity()
    }

    pub fn enqueue(&mut self, element: i32) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.elements.push_back(element);
    }

    pub fn front(&self) -> Option<i32> {
        self.elements.front().copied()
    }

    pub fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.elements.pop_front();
    }

    pub fn front_and_dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            self.elements.pop_front()
        }
    }
}

fn main() {
    let mut queue = Queue::new(10);
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    println!("Queue: {:?}", queue);

    if let Some(element) = queue.front() {
        println!("Front: {}", element);
    }

    queue.dequeue();

    println!("After dequeue: {:?}", queue);

    if let Some(element) = queue.front_and_dequeue() {
        println!("FrontAndDequeue: {}", element);
    }

    println!("After FrontAndDequeue: {:?}", queue);
}