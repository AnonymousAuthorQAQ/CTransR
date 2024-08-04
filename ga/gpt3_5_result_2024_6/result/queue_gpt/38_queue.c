use std::ptr;

struct QueueRecord {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: Vec<ElementType>,
}

type Queue = Box<QueueRecord>;

type ElementType = i32;

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
    unsafe { array.set_len(max_elements) };

    Box::new(QueueRecord {
        capacity: max_elements,
        front: 1,
        rear: 0,
        size: 0,
        array,
    })
}

fn make_empty(Q: &mut Queue) {
    Q.size = 0;
    Q.front = 1;
    Q.rear = 0;
}

fn dispose_queue(Q: Queue) {
    drop(Q);
}

fn succ(value: usize, Q: &Queue) -> usize {
    if value + 1 == Q.capacity {
        0
    } else {
        value + 1
    }
}

fn enqueue(X: ElementType, Q: &mut Queue) {
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

fn dequeue(Q: &mut Queue) {
    if is_empty(Q) {
        panic!("Empty queue");
    } else {
        Q.size -= 1;
        Q.front = succ(Q.front, Q);
    }
}

fn front_and_dequeue(Q: &mut Queue) -> ElementType {
    if is_empty(Q) {
        panic!("Empty queue");
    } else {
        let X = Q.array[Q.front];
        Q.size -= 1;
        Q.front = succ(Q.front, Q);
        X
    }
}

fn main() {
    let mut Q = create_queue(12);

    for i in 0..10 {
        enqueue(i, &mut Q);
    }

    while !is_empty(&Q) {
        println!("{}", front(&Q));
        dequeue(&mut Q);
    }

    for i in 0..10 {
        enqueue(i, &mut Q);
    }

    while !is_empty(&Q) {
        println!("{}", front(&Q));
        dequeue(&mut Q);
    }

    dispose_queue(Q);
}