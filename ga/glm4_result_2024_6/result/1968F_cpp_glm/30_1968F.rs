use std::io;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let n: i32 = iter.next().unwrap().parse().unwrap();
        let q: i32 = iter.next().unwrap().parse().unwrap();

        let mut arr: Vec<i32> = Vec::new();
        for _ in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let num: i32 = input.trim().parse().unwrap();
            arr.push(num);
        }
        
        let mut heap: BinaryHeap<PQItem> = BinaryHeap::new();
        let mut x: Vec<(i32, i32)> = Vec::new();
        x.push((0, 0));
        for i in 1..(n+1) {
            x.push((x[i-1].0 ^ arr[i-1], i));
            heap.push(PQItem::new(x[i-1]));
        }
        
        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.trim().split_whitespace();
            let l: i32 = iter.next().unwrap().parse().unwrap();
            let r: i32 = iter.next().unwrap().parse().unwrap();

            let c = x[r as usize].0 ^ x[(l-1) as usize].0;
            if c == 0 {
                println!("YES");
                continue;
            }

            let mut i1 = heap.binary_search(&PQItem::new((c ^ x[(l-1) as usize].0, l))).unwrap();
            if i1 == n as usize || x[i1].1 > r {
                println!("NO");
                continue;
            }

            let mut i2 = heap.binary_search(&PQItem::new((x[(l-1) as usize].0, x[i1].1+1))).unwrap();
            if i2 == n as usize || x[i2].1 > r || x[i2].0 != x[(l-1) as usize].0 {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}

#[derive(Eq, PartialEq)]
struct PQItem {
    val: i32,
    index: i32,
}

impl PQItem {
    fn new(vi: (i32, i32)) -> Self {
        PQItem { val: vi.0, index: vi.1 }
    }
}

impl Ord for PQItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for PQItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}