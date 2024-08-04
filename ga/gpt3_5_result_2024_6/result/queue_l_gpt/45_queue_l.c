use std::ptr;

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

    fn create_queue(max_elements: usize) -> Self {
        if max_elements < MinQueueSize {
            panic!("Queue size is too small");
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
            panic!("Full queue");
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
            panic!("Empty queue");
        }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        if self.is_empty() {
            panic!("Empty queue");
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

const MIN_QUEUE_SIZE: usize = 5;

fn main() {
    let queue_length: usize = match read_input() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid input or queue size is too small");
            std::process::exit(1);
        }
    };

    let mut Q = Queue::create_queue(queue_length);

    // Test: Attempt to dequeue from an empty queue
    if !Q.is_empty() {
        Q.dequeue();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    // Enqueue elements and check if the queue is full
    for i in 0..queue_length - 1 {
        if Q.is_full() {
            println!("Queue is full");
        } else {
            Q.enqueue(i);
        }
    }

    // Test: Attempt to enqueue to a full queue
    if !Q.is_full() {
        let enqueue_value: ElementType = match read_input() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Invalid input");
                std::process::exit(1);
            }
        };
        Q.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    // Use FrontAndDequeue to get and remove elements
    while !Q.is_empty() {
        println!("FrontAndDequeue: {}", Q.front_and_dequeue());
    }

    // Enqueue elements again
    for i in 0..queue_length - 2 {
        Q.enqueue(i);
    }

    // Use Front and Dequeue
    while !Q.is_empty() {
        println!("Front: {}", Q.front());
        Q.dequeue();
    }
}

fn read_input() -> Result<ElementType, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let trimmed_input = input.trim();
    let value: ElementType = trimmed_input.parse().unwrap();
    Ok(value)
}