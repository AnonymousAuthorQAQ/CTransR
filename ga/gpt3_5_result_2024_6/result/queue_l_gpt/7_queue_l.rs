use std::io;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
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

    fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.size += 1;
            self.rear = self.succ(self.rear);
            self.array[self.rear] = x;
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
            self.front = self.succ(self.front);
        }
    }

    fn front_and_dequeue(&mut self) -> i32 {
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

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read input");
    let queue_length: usize = input.trim().parse().expect("Invalid input");

    let mut Q = Queue::create_queue(queue_length);

    if Q.is_empty() {
        println!("Queue is empty, cannot dequeue.");
    }

    for i in 0..queue_length - 1 {
        if Q.is_full() {
            println!("Queue is full");
        } else {
            Q.enqueue(i as i32);
        }
    }

    if !Q.is_full() {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let enqueue_value: i32 = input.trim().parse().expect("Invalid input");
        Q.enqueue(enqueue_value);
    } else {
        println!("Queue is full, cannot enqueue.");
    }

    while !Q.is_empty() {
        println!("FrontAndDequeue: {}", Q.front_and_dequeue());
    }

    for i in 0..queue_length - 2 {
        Q.enqueue(i as i32);
    }

    while !Q.is_empty() {
        println!("Front: {}", Q.front());
        Q.dequeue();
    }
}