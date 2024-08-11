use std::ptr;

type ElementType = i32;

struct QueueRecord {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<ElementType>,
}

type Queue = Box<QueueRecord>;

const MIN_QUEUE_SIZE: usize = 5;

fn is_empty(Q: &Queue) -> bool {
    Q.size == 0
}

fn is_full(Q: &Queue) -> bool {
    Q.size == Q.capacity
}

fn create_queue(max_elements: usize) -> Queue {
    if max_elements < MIN_QUEUE_SIZE {
        panic!("Queue size is too small");
    }

    let mut array = Vec::with_capacity(max_elements);
    unsafe { array.set_len(max_elements); }

    let queue = Queue::new(QueueRecord {
        capacity: max_elements,
        front: 1,
        rear: 0,
        size: 0,
        array,
    });

    make_empty(&queue);
    queue
}

fn make_empty(Q: &Queue) {
    Q.size = 0;
    Q.front = 1;
    Q.rear = 0;
}

fn dispose_queue(Q: &mut Queue) {
    Q.array.clear();
}

fn succ(value: usize, Q: &Queue) -> usize {
    if value + 1 == Q.capacity {
        0
    } else {
        value + 1
    }
}

fn enqueue(X: ElementType, Q: &Queue) {
    if is_full(Q) {
        panic!("Full queue");
    } else {
        Q.size += 1;
        Q.rear = succ(Q.rear, Q);
        Q.array[Q.rear] = X;
    }
}

fn front(Q: &Queue) -> ElementType {
    if !is_empty(Q) {
        Q.array[Q.front]
    } else {
        panic!("Empty queue");
    }
}

fn dequeue(Q: &Queue) {
    if is_empty(Q) {
        panic!("Empty queue");
    } else {
        Q.size -= 1;
        Q.front = succ(Q.front, Q);
    }
}

fn front_and_dequeue(Q: &Queue) -> ElementType {
    let mut X = 0;

    if is_empty(Q) {
        panic!("Empty queue");
    } else {
        Q.size -= 1;
        X = Q.array[Q.front];
        Q.front = succ(Q.front, Q);
    }
    X
}

fn main() {
    let mut Q = create_queue(12);

    for i in 0..10 {
        enqueue(i, &Q);
    }

    while !is_empty(&Q) {
        println!("{}", front(&Q));
        dequeue(&Q);
    }

    for i in 0..10 {
        enqueue(i, &Q);
    }

    while !is_empty(&Q) {
        println!("{}", front(&Q));
        dequeue(&Q);
    }
}