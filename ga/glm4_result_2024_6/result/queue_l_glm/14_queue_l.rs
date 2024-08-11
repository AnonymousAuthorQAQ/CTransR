fn main() {
    let queue_length = get_queue_length();
    let mut queue = Queue::new(queue_length);

    // Test: Attempt to dequeue from an empty queue
    match queue.dequeue() {
        None => println!("Queue is empty, cannot dequeue."),
        _ => panic!("Queue should be empty"),
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            queue.enqueue(i);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !queue.is_full() {
        let enqueue_value = get_enqueue_value();
        queue.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while let Some(value) = queue.front_and_dequeue() {
        println!("FrontAndDequeue: {}", value);
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        queue.enqueue(i);
    }

    // Use Front and Dequeue
    while let Some(value) = queue.front() {
        println!("Front: {}", value);
        queue.dequeue();
    }
}

fn get_queue_length() -> usize {
    loop {
        println!("Enter the queue length (at least 5):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<usize>() {
            Ok(length) if length >= 5 => return length,
            _ => println!("Invalid input or queue size is too small. Please enter a number at least 5."),
        }
    }
}

fn get_enqueue_value() -> i32 {
    println!("Enter a value to enqueue:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse::<i32>() {
        Ok(value) => value,
        _ => {
            println!("Invalid input. Please enter a number.");
            get_enqueue_value()
        }
    }
}

struct Queue {
    capacity: usize,
    size: usize,
    front: usize,
    rear: usize,
    elements: Vec<i32>,
}

impl Queue {
    fn new(capacity: usize) -> Queue {
        Queue {
            capacity,
            size: 0,
            front: 0,
            rear: 0,
            elements: vec![0; capacity],
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn enqueue(&mut self, value: i32) {
        if self.is_full() {
            panic!("Queue is full");
        }
        self.size += 1;
        self.elements[self.rear] = value;
        self.rear = (self.rear + 1) % self.capacity;
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            let value = self.elements[self.front];
            self.front = (self.front + 1) % self.capacity;
            Some(value)
        }
    }

    fn front(&self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            Some(self.elements[self.front])
        }
    }

    fn front_and_dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            let value = self.elements[self.front];
            self.front = (self.front + 1) % self.capacity;
            Some(value)
        }
    }
}