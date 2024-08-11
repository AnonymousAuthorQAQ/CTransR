use std::ptr::NonNull;

struct QueueRecord {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: NonNull<ElementType>,
}

type Queue = NonNull<QueueRecord>;

type ElementType = i32;

const MIN_QUEUE_SIZE: usize = 5;

fn is_empty(q: &QueueRecord) -> bool {
    q.size == 0
}

fn is_full(q: &QueueRecord) -> bool {
    q.size == q.capacity
}

fn create_queue(max_elements: usize) -> Queue {
    if max_elements < MIN_QUEUE_SIZE {
        panic!("Queue size is too small");
    }
    
    let array = unsafe {
        NonNull::new_unchecked(Box::into_raw(vec![0; max_elements].into_boxed_slice()) as *mut ElementType)
    };
    
    let mut q = Box::new(QueueRecord {
        capacity: max_elements,
        front: 1,
        rear: 0,
        size: 0,
        array,
    });
    
    let q_ptr = NonNull::from(&mut *q);
    
    std::mem::forget(q);
    
    q_ptr
}

fn make_empty(q: &mut QueueRecord) {
    q.size = 0;
    q.front = 1;
    q.rear = 0;
}

fn dispose_queue(q: &mut Queue) {
    unsafe {
        Box::from_raw((*q.as_ptr()).array.as_ptr());
        Box::from_raw(q.as_ptr());
    }
}

fn succ(value: usize, q: &QueueRecord) -> usize {
    if value + 1 == q.capacity {
        0
    } else {
        value + 1
    }
}

fn enqueue(x: ElementType, q: &mut QueueRecord) {
    if is_full(q) {
        panic!("Full queue");
    } else {
        q.size += 1;
        q.rear = succ(q.rear, q);
        q.array.as_ptr()[q.rear] = x;
    }
}

fn front(q: &QueueRecord) -> ElementType {
    if !is_empty(q) {
        q.array.as_ptr()[q.front]
    } else {
        panic!("Empty queue");
    }
}

fn dequeue(q: &mut QueueRecord) {
    if is_empty(q) {
        panic!("Empty queue");
    } else {
        q.size -= 1;
        q.front = succ(q.front, q);
    }
}

fn front_and_dequeue(q: &mut QueueRecord) -> ElementType {
    if is_empty(q) {
        panic!("Empty queue");
    } else {
        q.size -= 1;
        let x = q.array.as_ptr()[q.front];
        q.front = succ(q.front, q);
        x
    }
}

fn main() {
    let mut q = create_queue(12);

    for i in 0..10 {
        enqueue(i, &mut *q.as_ptr());
    }

    while !is_empty(&*q.as_ptr()) {
        println!("{}", front(&*q.as_ptr()));
        dequeue(&mut *q.as_ptr());
    }

    for i in 0..10 {
        enqueue(i, &mut *q.as_ptr());
    }

    while !is_empty(&*q.as_ptr()) {
        println!("{}", front(&*q.as_ptr()));
        dequeue(&mut *q.as_ptr());
    }

    dispose_queue(&mut q);
}