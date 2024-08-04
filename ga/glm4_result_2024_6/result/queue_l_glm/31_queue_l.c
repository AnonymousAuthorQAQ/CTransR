use std::collections::VecDeque;

type ElementType = i32;

const MIN_QUEUE_SIZE: usize = 5;

struct Queue {
    elements: VecDeque<ElementType>,
    capacity: usize,
}

impl Queue {
    fn new(capacity: usize) -> Queue {
        if capacity < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }
        Queue {
            elements: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn is_full(&self) -> bool {
        self.elements.len() == self.capacity
    }

    fn enqueue(&mut self, element: ElementType) {
        if self.is_full() {
            panic!("Queue is full");
        }
        self.elements.push_back(element);
    }

    fn dequeue(&mut self) -> Option<ElementType> {
        self.elements.pop_front()
    }

    fn front(&self) -> Option<ElementType> {
        self.elements.front().copied()
    }
}

fn main() {
    let mut queue_capacity: usize;
    println!("Enter the queue capacity:");
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");
    queue_capacity = queue_capacity.trim().parse().expect("Please type a number");

    if queue_capacity < MIN_QUEUE_SIZE {
        println!("Queue size is too small");
        return;
    }

    let mut queue = Queue::new(queue_capacity);

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        println!("Dequeue: {:?}", queue.dequeue());
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_capacity - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        println!("Enter an element to enqueue:");
        let mut element = String::new();
        std::io::stdin()
            .read_line(&mut element)
            .expect("Failed to read line");
        let element: ElementType = element.trim().parse().expect("Please type a number");
        queue.enqueue(element);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use front and dequeue
    while !queue.is_empty() {
        println!("Front: {:?}", queue.front());
        queue.dequeue();
    }

    // Enqueue elements again
    for i in 0..queue_capacity - 2 {
        queue.enqueue(i);
    }

    // Use front and dequeue
    while !queue.is_empty() {
        println!("Front: {:?}", queue.front());
        queue.dequeue();
    }
}