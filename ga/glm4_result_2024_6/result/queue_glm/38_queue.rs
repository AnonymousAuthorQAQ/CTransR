use std::collections::VecDeque;

fn main() {
    let mut q: VecDeque<i32> = VecDeque::with_capacity(12);

    for i in 0..10 {
        q.push_back(i);
    }

    while let Some(x) = q.pop_front() {
        println!("{}", x);
    }

    for i in 0..10 {
        q.push_back(i);
    }

    while let Some(x) = q.pop_front() {
        println!("{}", x);
    }
}