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
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

fn create_queue(max_elements: usize) -> QueueRecord {
    if max_elements < 5 {
        panic!("Queue size is too small");
    }

    QueueRecord {
        capacity: max_elements,
        front: 1,
        rear: 0,
        size: 0,
        array: vec![0; max_elements],
    }
}

fn enqueue(x: ElementType, q: &mut QueueRecord) {
    if q.is_full() {
        panic!("Full queue");
    } else {
        q.size += 1;
        q.rear = succ(q.rear, q);
        q.array[q.rear] = x;
    }
}

fn front(q: &QueueRecord) -> ElementType {
    if !q.is_empty() {
        q.array[q.front]
    } else {
        panic!("Empty queue");
    }
}

fn dequeue(q: &mut QueueRecord) {
    if q.is_empty() {
        panic!("Empty queue");
    } else {
        q.size -= 1;
        q.front = succ(q.front, q);
    }
}

fn front_and_dequeue(q: &mut QueueRecord) -> ElementType {
    if q.is_empty() {
        panic!("Empty queue");
    } else {
        let x = q.array[q.front];
        q.size -= 1;
        q.front = succ(q.front, q);
        x
    }
}

fn succ(value: usize, q: &QueueRecord) -> usize {
    if value + 1 == q.capacity {
        0
    } else {
        value + 1
    }
}

fn main() {
    let mut q = create_queue(12);

    for i in 0..10 {
        enqueue(i, &mut q);
    }

    while !q.is_empty() {
        println!("{}", front(&q));
        dequeue(&mut q);
    }

    for i in 0..10 {
        enqueue(i, &mut q);
    }

    while !q.is_empty() {
        println!("{}", front(&q));
        dequeue(&mut q);
    }
}