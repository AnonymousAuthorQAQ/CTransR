use std::io;

type ElementType = i32;

struct QueueRecord {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<ElementType>,
}

impl QueueRecord {
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn create_queue(max_elements: usize) -> Self {
        let array = vec![0; max_elements];
        QueueRecord {
            capacity: max_elements,
            front: 0,
            rear: max_elements - 1,
            size: 0,
            array,
        }
    }

    fn enqueue(&mut self, x: ElementType) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.rear = self.rear + 1 % self.capacity;
            self.array[self.rear] = x;
        }
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
        } else {
            self.size -= 1;
            self.front = self.front + 1 % self.capacity;
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        let x;
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.size -= 1;
            x = self.array[self.front];
            self.front = self.front + 1 % self.capacity;
        }
        x
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let queue_length: usize = input.trim().parse().unwrap();

    let mut Q = QueueRecord::create_queue(queue_length);

    if Q.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    }

    for i in 0..queue_length - 1 {
        if Q.is_full() {
            println!("Queue is full");
        } else {
            Q.enqueue(i as ElementType);
        }
    }

    if !Q.is_full() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let enqueue_value: ElementType = input.trim().parse().unwrap();
        Q.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    while !Q.is_empty() {
        println!("FrontAndDequeue: {}", Q.front_and_dequeue());
    }

    for i in 0..queue_length - 2 {
        Q.enqueue(i as ElementType);
    }

    while !Q.is_empty() {
        println!("Front: {}", Q.front());
        Q.dequeue();
    }
}