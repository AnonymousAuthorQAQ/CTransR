use std::ptr;

struct QueueRecord {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<ElementType>,
}

struct Queue {
    queue_record: QueueRecord,
}

type ElementType = i32;

const MIN_QUEUE_SIZE: usize = 5;

impl Queue {
    fn is_empty(&self) -> bool {
        self.queue_record.size == 0
    }

    fn is_full(&self) -> bool {
        self.queue_record.size == self.queue_record.capacity
    }

    fn create_queue(max_elements: usize) -> Queue {
        if max_elements < MIN_QUEUE_SIZE {
            panic!("Queue size is too small");
        }

        let array = Vec::with_capacity(max_elements);
        Queue {
            queue_record: QueueRecord {
                capacity: max_elements,
                front: 1,
                rear: 0,
                size: 0,
                array,
            },
        }
    }

    fn make_empty(&mut self) {
        self.queue_record.size = 0;
        self.queue_record.front = 1;
        self.queue_record.rear = 0;
    }

    fn dispose_queue(&mut self) {
        self.queue_record.array.clear();
        self.queue_record.capacity = 0;
    }

    fn succ(&self, value: usize) -> usize {
        if value + 1 == self.queue_record.capacity {
            0
        } else {
            value + 1
        }
    }

    fn enqueue(&mut self, x: ElementType) {
        if self.is_full() {
            panic!("Full queue");
        } else {
            self.queue_record.size += 1;
            self.queue_record.rear = self.succ(self.queue_record.rear);
            self.queue_record.array.push(x);
        }
    }

    fn front(&self) -> ElementType {
        if !self.is_empty() {
            self.queue_record.array[self.queue_record.front]
        } else {
            panic!("Empty queue");
        }
    }

    fn dequeue(&mut self) {
        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.queue_record.size -= 1;
            self.queue_record.front = self.succ(self.queue_record.front);
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        let x: ElementType;

        if self.is_empty() {
            panic!("Empty queue");
        } else {
            self.queue_record.size -= 1;
            x = self.queue_record.array[self.queue_record.front];
            self.queue_record.front = self.succ(self.queue_record.front);
        }
        x
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

    Q.dispose_queue();
}