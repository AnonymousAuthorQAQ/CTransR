use std::ptr;
use std::process;

type ElementType = i32;

struct QueueRecord {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Box<[ElementType]>,
}

struct Queue {
    queue_record: QueueRecord,
}

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
            eprintln!("Queue size is too small");
            process::exit(1);
        }

        let array = vec![0; max_elements].into_boxed_slice();

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

    fn enqueue(&mut self, x: ElementType) {
        if self.is_full() {
            eprintln!("Full queue");
            process::exit(1);
        } else {
            let rear = (self.queue_record.rear + 1) % self.queue_record.capacity;
            self.queue_record.size += 1;
            self.queue_record.rear = rear;
            self.queue_record.array[self.queue_record.rear] = x;
        }
    }

    fn front(&self) -> ElementType {
        if !self.is_empty() {
            self.queue_record.array[self.queue_record.front]
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
            self.queue_record.size -= 1;
            self.queue_record.front = (self.queue_record.front + 1) % self.queue_record.capacity;
        }
    }

    fn front_and_dequeue(&mut self) -> ElementType {
        let x: ElementType;
        
        if self.is_empty() {
            eprintln!("Empty queue");
            process::exit(1);
        } else {
            self.queue_record.size -= 1;
            x = self.queue_record.array[self.queue_record.front];
            self.queue_record.front = (self.queue_record.front + 1) % self.queue_record.capacity;
        }
        
        x
    }

    fn dispose_queue(&mut self) {
        self.queue_record.size = 0;
        self.queue_record.front = 0;
        self.queue_record.rear = self.queue_record.capacity - 1;
        self.queue_record.array = Box::new([]);
    }
}

fn main() {
    let mut q = Queue::create_queue(12);

    for i in 0..10 {
        q.enqueue(i);
    }

    while !q.is_empty() {
        println!("{}", q.front());
        q.dequeue();
    }

    for i in 0..10 {
        q.enqueue(i);
    }

    while !q.is_empty() {
        println!("{}", q.front());
        q.dequeue();
    }

    q.dispose_queue();
}