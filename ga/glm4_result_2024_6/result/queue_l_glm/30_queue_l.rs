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
    fn new(capacity: usize) -> Queue {
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
        let front = self.front();
        self.dequeue();
        front
    }
}

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Queue {{ capacity: {}, front: {}, rear: {}, size: {}, array: {:?} }}", self.capacity, self.front, self.rear, self.size, self.array)
    }
}

fn main() {
    let mut queue: Queue;
    let mut queue_length: usize;

    loop {
        println!("Enter the queue length (must be at least 5):");
        queue_length = {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse() {
                Ok(length) => length,
                Err(_) => {
                    println!("Invalid input. Please enter a number.");
                    continue;
                }
            }
        };

        if queue_length < 5 {
            println!("Queue size is too small. Please enter a number at least 5.");
        } else {
            break;
        }
    }

    queue = Queue::new(queue_length);

    // Test: Attempt to dequeue from an empty queue
    if !queue.is_empty() {
        queue.dequeue();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i as i32);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        println!("Enter a value to enqueue:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let enqueue_value: i32 = input.trim().parse().expect("Invalid input");
        queue.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !queue.is_empty() {
        println!("FrontAndDequeue: {}", queue.front_and_dequeue());
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i as i32);
    }

    // Use Front and Dequeue
    while !queue.is_empty() {
        println!("Front: {}", queue.front());
        queue.dequeue();
    }

    println!("Final queue: {}", queue);
}