use std::ptr;

type ElementType = i32;

struct Queue {
    capacity: i32,
    front: i32,
    rear: i32,
    size: i32,
    array: Vec<ElementType>,
}

const MIN_QUEUE_SIZE: i32 = 5;

impl Queue {
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn create_queue(max_elements: i32) -> Self {
        if max_elements < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }
        
        Queue {
            capacity: max_elements,
            front: 1,
            rear: 0,
            size: 0,
            array: vec![0; max_elements as usize],
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
            self.array[self.rear as usize] = x;
        }
    }

    fn front(&self) -> ElementType {
        if !self.is_empty() {
            self.array[self.front as usize]
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
            let x = self.array[self.front as usize];
            self.front = self.succ(self.front);
            x
        }
    }

    fn succ(&self, value: i32) -> i32 {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }
}

fn main() {
    let mut Q = Queue::create_queue(12);

    for i in 0..10 {
        Q.enqueue(i);
    }

    while !Q.is_empty() {
        println!("{}", Q.front());
        Q.dequeue();
    }

    for i in 0..10 {
        Q.enqueue(i);
    }

    while !Q.is_empty() {
        println!("{}", Q.front());
        Q.dequeue();
    }
}