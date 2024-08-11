use std::ptr;

type ElementType = i32;

struct QueueRecord {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<ElementType>,
}

impl QueueRecord {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            front: 1,
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

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.capacity {
            0
        } else {
            value + 1
        }
    }

}


struct Queue(QueueRecord);

impl Queue {
    fn create_queue(max_elements: usize) -> Self {
        if max_elements < 5 {
            panic!("Queue size is too small");
        }

        Queue(QueueRecord::new(max_elements))
    }

    fn make_empty(&mut self) {
        self.0.size = 0;
        self.0.front = 1;
        self.0.rear = 0;
    }

    fn enqueue(&mut self, x: ElementType) {
        if self.0.is_full() {
            panic!("Full queue");
        } else {
            self.0.size += 1;
            self.0.rear = self.0.succ(self.0.rear);
            self.0.array[self.0.rear] = x;
        }
    }

    fn front(&self) -> ElementType {
        if !self.0.is_empty() {
            self.0.array[self.0.front]
        } else {
            println!("Empty queue");
            0
        }
    }

    fn dequeue(&mut self) {
        if self.0.is_empty() {
            panic!("Empty queue");
        } else {
            self.0.size -= 1;
            self.0.front = self.0.succ(self.0.front);
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        let x;
        
        if self.0.is_empty() {
            panic!("Empty queue");
        } else {
            self.0.size -= 1;
            x = self.0.array[self.0.front];
            self.0.front = self.0.succ(self.0.front);
        }

        x
    }

    fn dispose_queue(&mut self) {
        unsafe {
            ptr::drop_in_place(&mut self.0.array);
        }
    }
}

fn main() {
    let mut Q = Queue::create_queue(12);

    for i in 0..10 {
        Q.enqueue(i);
    }

    while !Q.0.is_empty() {
        println!("{}", Q.front());
        Q.dequeue();
    }

    for i in 0..10 {
        Q.enqueue(i);
    }

    while !Q.0.is_empty() {
        println!("{}", Q.front());
        Q.dequeue();
    }

    Q.dispose_queue();
}