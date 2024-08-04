use std::ptr;
use std::process;

type ElementType = i32;

struct QueueRecord {
    capacity: usize,
    front: usize,
    rear: usize,
    size: usize,
    array: *mut ElementType,
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

    let mut Q: Queue = Box::into_raw(Box::new(QueueRecord {
        capacity: max_elements,
        front: 1,
        rear: 0,
        size: 0,
        array: ptr::null_mut(),
    }));

    let array = Box::into_raw(vec![0; max_elements]);
    unsafe {
        (*Q).array = array;
    }

    Q
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
        if !(*Q).array.is_null() {
            Box::from_raw((*Q).array);
        }
        Box::from_raw(Q);
    }
}

fn succ(value: usize, Q: Queue) -> usize {
    if value + 1 == (*Q).capacity {
        return 0;
    }
    value + 1
}

fn enqueue(X: ElementType, Q: Queue) {
    if is_full(Q) {
        panic!("Full queue");
    } else {
        unsafe {
            (*Q).size += 1;
            (*Q).rear = succ((*Q).rear, Q);
            (*(*Q).array.offset((*Q).rear as isize)) = X;
        }
    }
}

fn front(Q: Queue) -> ElementType {
    if !is_empty(Q) {
        unsafe {
            return *(*(*Q).array.offset((*Q).front as isize));
        }
    }
    panic!("Empty queue");
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

fn front_and_dequeue(Q: Queue) -> ElementType {
    let mut X = 0;

    if is_empty(Q) {
        panic!("Empty queue");
    } else {
        unsafe {
            (*Q).size -= 1;
            X = *(*(*Q).array.offset((*Q).front as isize));
            (*Q).front = succ((*Q).front, Q);
        }
    }
    X
}

fn main() {
    let mut Q = create_queue(12);

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