use std::fmt;

const MIN_QUEUE_SIZE: usize = 5;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<Option<i32>>,
}

impl Queue {
    fn new(capacity: usize) -> Self {
        if capacity < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }
        Queue {
            capacity,
            front: 0,
            rear: 0,
            size: 0,
            array: vec![None; capacity],
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
        self.array[self.rear] = Some(element);
        self.rear = (self.rear + 1) % self.capacity;
    }

    fn front(&self) -> Option<i32> {
        self.array[self.front].clone()
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        }
        self.size -= 1;
        self.front = (self.front + 1) % self.capacity;
    }
}

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut current = self.front;
        for _ in 0..self.size {
            s.push_str(&format!("{}, ", self.array[current]));
            current = (current + 1) % self.capacity;
        }
        write!(f, "[{}]", s.trim_end_matches(", "))
    }
}

fn main() {
    let mut queue: Queue;
    let mut queue_length = 0;

    println!("Enter the queue size: ");
    std::io::stdin()
        .read_line(&mut queue_length)
        .expect("Failed to read line");
    queue_length = queue_length.trim().parse().expect("Invalid input");

    if queue_length < MIN_QUEUE_SIZE {
        println!("Queue size is too small");
        return;
    }

    queue = Queue::new(queue_length);

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        queue.enqueue(i);
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        println!("Enter a value to enqueue: ");
        let mut value = String::new();
        std::io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");
        let value: i32 = value.trim().parse().expect("Invalid input");
        queue.enqueue(value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front().unwrap());
        queue.dequeue();
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i);
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front().unwrap());
        queue.dequeue();
    }

    println!("Final Queue: {}", queue);
}