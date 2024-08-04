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
        assert!(capacity >= 5, "Queue size is too small");
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

    fn enqueue(&mut self, elem: i32) {
        if self.is_full() {
            panic!("Full queue");
        }
        self.size += 1;
        self.array[self.rear] = elem;
        self.rear = (self.rear + 1) % self.capacity;
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            let elem = self.array[self.front];
            self.front = (self.front + 1) % self.capacity;
            Some(elem)
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
        let mut queue = self.clone();
        write!(f, "Queue{{ capacity: {}, size: {}, elements: [", self.capacity, self.size)?;
        while let Some(elem) = queue.dequeue() {
            write!(f, "{}, ", elem)?;
        }
        write!(f, "] }}")
    }
}

fn main() {
    let mut queue = Queue::new(12);
    for i in 0..10 {
        queue.enqueue(i);
    }

    println!("Queue after enqueueing: {}", queue);

    while !queue.is_empty() {
        println!("Dequeue: {}", queue.dequeue().unwrap());
    }

    for i in 0..10 {
        queue.enqueue(i);
    }

    println!("Queue after re-enqueueing: {}", queue);

    while !queue.is_empty() {
        println!("Dequeue: {}", queue.dequeue().unwrap());
    }
}