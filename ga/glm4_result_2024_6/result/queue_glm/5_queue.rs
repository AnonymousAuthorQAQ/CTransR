use std::fmt;

const MIN_QUEUE_SIZE: usize = 5;

struct Queue {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<i32>,
}

impl Queue {
    fn new(capacity: usize) -> Queue {
        if capacity < MIN_QUEUE_SIZE {
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
            panic!("Queue is full");
        }
        self.size += 1;
        self.array[self.rear] = element;
        self.rear = (self.rear + 1) % self.capacity;
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            let result = self.array[self.front];
            self.front = (self.front + 1) % self.capacity;
            Some(result)
        }
    }

    fn front(&self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            Some(self.array[self.front])
        }
    }
}

impl fmt::Display for Queue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut queue_str = String::new();
        let mut current = self.front;
        for _ in 0..self.size {
            queue_str.push_str(&format!("{} ", self.array[current]));
            current = (current + 1) % self.capacity;
        }
        write!(f, "{}", queue_str.trim())
    }
}

fn main() {
    let mut queue = Queue::new(12);
    for i in 0..10 {
        queue.enqueue(i);
    }
    println!("Queue: {}", queue);

    while let Some(dequeued) = queue.dequeue() {
        println!("Dequeued: {}", dequeued);
    }

    for i in 0..10 {
        queue.enqueue(i);
    }
    println!("Queue: {}", queue);

    while let Some(dequeued) = queue.dequeue() {
        println!("Dequeued: {}", dequeued);
    }
}