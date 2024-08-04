use std::fmt;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    fn new(max_elements: usize) -> Queue {
        if max_elements < 5 {
            panic!("Queue size is too small");
        }
        Queue {
            capacity: max_elements,
            front: 0,
            rear: 0,
            size: 0,
            array: vec![0; max_elements],
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.array[self.rear] = x;
            self.rear = (self.rear + 1) % self.capacity;
        }
    }

    fn front(&self) -> i32 {
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
            self.front = (self.front + 1) % self.capacity;
        }
    }

    fn front_and_dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            let x = self.array[self.front];
            self.size -= 1;
            self.front = (self.front + 1) % self.capacity;
            x
        }
    }
}

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut current = self.front;
        for _ in 0..self.size {
            s.push_str(&format!("{} ", self.array[current]));
            current = (current + 1) % self.capacity;
        }
        write!(f, "{}", s.trim())
    }
}

fn main() {
    let mut queue_length: usize;
    loop {
        println!("Enter the queue length (minimum 5):");
        queue_length = std::io::stdin()
            .read_line()
            .expect("Failed to read line")
            .trim()
            .parse()
            .expect("Please enter a valid number");
        if queue_length < 5 {
            println!("Queue size is too small, try again.");
        } else {
            break;
        }
    }

    let mut q = Queue::new(queue_length);

    if !q.is_empty() {
        q.dequeue();
    } else {
        println!("Queue is empty, cannot dequeue.");
    }

    for i in 0..queue_length - 1 {
        if q.is_full() {
            println!("Queue is full");
        } else {
            q.enqueue(i);
        }
    }

    let mut enqueue_value: i32;
    loop {
        println!("Enter a value to enqueue (or type 'done' to continue):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "done" {
            break;
        }
        enqueue_value = input.trim().parse().expect("Please enter a valid number");
        if !q.is_full() {
            q.enqueue(enqueue_value);
        } else {
            println!("Queue is full, cannot enqueue.");
        }
    }

    while !q.is_empty() {
        println!("FrontAndDequeue: {}", q.front_and_dequeue());
    }

    for i in 0..queue_length - 2 {
        q.enqueue(i);
    }

    while !q.is_empty() {
        println!("Front: {}", q.front());
        q.dequeue();
    }

    println!("Final Queue: {}", q);
}