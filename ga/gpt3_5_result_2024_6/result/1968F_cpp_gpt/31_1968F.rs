use std::cmp::Ordering;
use std::io;

#[derive(Debug, Copy, Clone)]
struct Pair {
    first: i32,
    second: i32,
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first
    }
}

impl Eq for Pair {}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.first.cmp(&other.first)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n: i32 = parts.next().unwrap().parse().unwrap();
        let q: i32 = parts.next().unwrap().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let arr: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let mut x: Vec<Pair> = vec![Pair { first: 0, second: 0 }; (n + 1) as usize];
        let mut x2: Vec<i32> = vec![0; (n + 1) as usize];

        x[0] = Pair { first: 0, second: 0 };
        for i in 1..=n {
            x[i as usize] = Pair { first: x[(i - 1) as usize].first ^ arr[(i - 1) as usize], second: i };
        }

        for i in 0..=n {
            x2[i as usize] = x[i as usize].first;
        }

        x.sort();

        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let mut qr = input.split_whitespace().map(|x| x.parse().unwrap());
            let l: i32 = qr.next().unwrap();
            let r: i32 = qr.next().unwrap();

            let c = x2[r as usize] ^ x2[(l - 1) as usize];
            if c == 0 {
                println!("YES");
                continue;
            }

            let mut i1 = x.binary_search_by(|&val| val.cmp(&Pair { first: c ^ x2[(l - 1) as usize], second: l }));
            if let Err(idx) = i1 {
                i1 = Ok(idx);
            }

            if i1 == Ok(n as usize + 1) || (x[i1.unwrap()].second > r) || (x[i1.unwrap()].first != (c ^ x2[(l - 1) as usize])) {
                println!("NO");
                continue;
            }

            let mut i2 = x.binary_search_by(|&val| val.cmp(&Pair { first: x2[(l - 1) as usize], second: x[i1.unwrap()].second + 1 }));
            if let Err(idx2) = i2 {
                i2 = Ok(idx2);
            }

            if i2 == Ok(n as usize + 1) || (x[i2.unwrap()].second > r) || (x[i2.unwrap()].first != x2[(l - 1) as usize]) {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}