use std::ptr;
use std::alloc::{alloc, dealloc, Layout};

struct QueueRecord {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut i32,
}

type Queue = *mut QueueRecord;

const MIN_QUEUE_SIZE: usize = 5;

fn is_empty(Q: Queue) -> bool {
    unsafe { (*Q).size == 0 }
}

fn is_full(Q: Queue) -> bool {
    unsafe { (*Q).size == (*Q).capacity }
}

fn create_queue(max_elements: usize) -> Queue {
    if max_elements < MIN_QUEUE_SIZE {
        panic!("Queue size is too small");
    }

    let layout = Layout::array::<i32>(max_elements).unwrap();
    let array = unsafe { alloc(layout) as *mut i32 };

    let mut queue = Box::new(QueueRecord {
        capacity: max_elements,
        front: 1,
        rear: 0,
        size: 0,
        array,
    });

    let queue_raw = Box::into_raw(queue);
    queue_raw
}

fn make_empty(Q: Queue) {
    unsafe {
        (*Q).size = 0;
        (*Q).front = 1;
        (*Q).rear = 0;
    }
}

fn dispose_queue(Q: Queue) {
    unsafe {
        dealloc((*Q).array as *mut u8, Layout::array::<i32>((*Q).capacity).unwrap());
    }
    drop(Q);
}

fn succ(value: usize, Q: Queue) -> usize {
    if value + 1 == (*Q).capacity {
        0
    } else {
        value + 1
    }
}

fn enqueue(X: i32, Q: Queue) {
    if is_full(Q) {
        panic!("Full queue");
    } else {
        unsafe {
            (*Q).size += 1;
            (*Q).rear = succ((*Q).rear, Q);
            (*(*Q).array.add((*Q).rear)) = X;
        }
    }
}

fn front(Q: Queue) -> i32 {
    if !is_empty(Q) {
        unsafe {
            *((*Q).array.add((*Q).front))
        }
    } else {
        panic!("Empty queue");
    }
}

fn dequeue(Q: Queue) {
    if is_empty(Q) {
        panic!("Empty queue");
    } else {
        unsafe {
            (*Q).size -= 1;
            (*Q).front = succ((*Q).front, Q);
        }
    }
}

fn front_and_dequeue(Q: Queue) -> i32 {
    let mut X = 0;

    if is_empty(Q) {
        panic!("Empty queue");
    } else {
        unsafe {
            (*Q).size -= 1;
            X = *((*Q).array.add((*Q).front));
            (*Q).front = succ((*Q).front, Q);
        }
    }

    X
}

fn main() {
    let Q = create_queue(12);

    for i in 0..10 {
        enqueue(i, Q);
    }

    while !is_empty(Q) {
        println!("{}", front(Q));
        dequeue(Q);
    }

    for i in 0..10 {
        enqueue(i, Q);
    }

    while !is_empty(Q) {
        println!("{}", front(Q));
        dequeue(Q);
    }

    dispose_queue(Q);
}