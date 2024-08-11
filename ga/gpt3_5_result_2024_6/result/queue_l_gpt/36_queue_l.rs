use std::ptr;
use std::process;

type ElementType = i32;

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

    fn create_queue(max_elements: usize) -> Queue {
        if max_elements < 5 {
            eprintln!("Queue size is too small");
            process::exit(1);
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
            eprintln!("Full queue");
            process::exit(1);
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
            eprintln!("Empty queue");
            process::exit(1);
        }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            eprintln!("Empty queue");
            process::exit(1);
        } else {
            self.size -= 1;
            self.front = self.succ(self.front);
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        let x: ElementType;

        if self.is_empty() {
            eprintln!("Empty queue");
            process::exit(1);
        } else {
            self.size -= 1;
            x = self.array[self.front];
            self.front = self.succ(self.front);
        }

        x
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }

    fn dispose_queue(&mut self) {
        self.array = vec![];
    }
}

fn main() {
    let mut queue_length = String::new();
    std::io::stdin().read_line(&mut queue_length)
        .expect("Failed to read line");
    let mut queue_length: usize = queue_length.trim().parse()
        .expect("Invalid input or queue size is too small");

    let mut Q = Queue::create_queue(queue_length);

    if Q.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    }

    for i in 0..queue_length-1 {
        if Q.is_full() {
            println!("Queue is full");
        } else {
            Q.enqueue(i as ElementType);
        }
    }

    if !Q.is_full() {
        let mut enqueue_value = String::new();
        std::io::stdin().read_line(&mut enqueue_value)
            .expect("Failed to read line");
        let enqueue_value: ElementType = enqueue_value.trim().parse()
            .expect("Invalid input");
        Q.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    while !Q.is_empty() {
        println!("FrontAndDequeue: {}", Q.front_and_dequeue());
    }

    for i in 0..queue_length-2 {
        Q.enqueue(i as ElementType);
    }

    while !Q.is_empty() {
        println!("Front: {}", Q.front());
        Q.dequeue();
    }

    Q.dispose_queue();
}