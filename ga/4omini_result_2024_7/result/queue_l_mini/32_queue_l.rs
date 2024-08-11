use std::io;
use std::ptr;

#[derive(Debug)]
struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    fn new(max_elements: usize) -> Result<Self, String> {
        if max_elements < 5 {
            return Err("Queue size is too small".to_string());
        }

        Ok(Self {
            capacity: max_elements,
            front: 1,
            rear: 0,
            size: 0,
            array: vec![0; max_elements],
        })
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn make_empty(&mut self) {
        self.size = 0;
        self.front = 1;
        self.rear = 0;
    }

    fn enqueue(&mut self, x: i32) -> Result<(), String> {
        if self.is_full() {
            return Err("Full queue".to_string());
        }
        self.size += 1;
        self.rear = self.successor(self.rear);
        self.array[self.rear] = x;
        Ok(())
    }

    fn front(&self) -> Result<i32, String> {
        if !self.is_empty() {
            Ok(self.array[self.front])
        } else {
            Err("Empty queue".to_string())
        }
    }

    fn dequeue(&mut self) -> Result<(), String> {
        if self.is_empty() {
            return Err("Empty queue".to_string());
        }
        self.size -= 1;
        self.front = self.successor(self.front);
        Ok(())
    }

    fn front_and_dequeue(&mut self) -> Result<i32, String> {
        let x = self.front()?;
        self.size -= 1;
        self.front = self.successor(self.front);
        Ok(x)
    }

    fn successor(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }
}

fn main() {
    let mut queue_length = String::new();

    println!("Enter queue length: ");
    io::stdin().read_line(&mut queue_length).expect("Failed to read input");
    
    let queue_length: usize = queue_length.trim().parse().expect("Invalid input");
    if queue_length < 5 {
        eprintln!("Queue size is too small");
        return;
    }

    let mut queue = Queue::new(queue_length).expect("Failed to create queue");

    if queue.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    } else {
        if let Err(e) = queue.dequeue() {
            eprintln!("{}", e);
        }
    }
    
    for i in 0..(queue_length - 1) {
        if queue.is_full() {
            println!("Queue is full");
        } else {
            let _ = queue.enqueue(i).expect("Enqueue operation failed");
        }
    }
    
    let mut enqueue_value = String::new();
    
    if queue.is_full() {
        println!("Queue is full, cannot enqueue.");
    } else {
        println!("Enter value to enqueue: ");
        io::stdin().read_line(&mut enqueue_value).expect("Failed to read input");
        let enqueue_value: i32 = enqueue_value.trim().parse().expect("Invalid input");
        let _ = queue.enqueue(enqueue_value).expect("Enqueue operation failed");
    }

    while !queue.is_empty() {
        match queue.front_and_dequeue() {
            Ok(value) => println!("FrontAndDequeue: {}", value),
            Err(e) => eprintln!("{}", e),
        }
    }

    for i in 0..(queue_length - 2) {
        let _ = queue.enqueue(i).expect("Enqueue operation failed");
    }

    while !queue.is_empty() {
        match queue.front() {
            Ok(value) => {
                println!("Front: {}", value);
                let _ = queue.dequeue().expect("Dequeue operation failed");
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}