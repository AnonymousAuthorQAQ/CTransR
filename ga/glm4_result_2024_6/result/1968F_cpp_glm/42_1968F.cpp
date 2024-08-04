use std::io;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    while t > 0 {
        t -= 1;
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let v: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        let n = v[0] as usize;
        let q = v[1] as usize;
        let mut arr = vec![0; n];
        let mut x = vec![(0, 0); n + 1];
        let mut x2 = vec![0; n + 1];

        for i in 0..n {
            io::stdin().read_line(&mut input).unwrap();
            arr[i] = input.trim().parse().unwrap();
        }

        for i in 1..=n {
            x[i] = (x[i - 1].0 ^ arr[i - 1], i);
            x2[i] = x[i].0;
        }
        x.sort_by(|a, b| {
            if a.0 == b.0 {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        for _ in 0..q {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let v: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
            let l = v[0] as usize;
            let r = v[1] as usize;

            let c = x2[r] ^ x2[l - 1];
            if c == 0 {
                println!("YES");
                continue;
            }

            let mut i1 = 0;
            while i1 < n + 1 && x[i1] < (c ^ x2[l - 1], l) {
                i1 += 1;
            }

            if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
                println!("NO");
                continue;
            }

            let mut i2 = 0;
            while i2 < n + 1 && x[i2] < (x2[l - 1], x[i1].1 + 1) {
                i2 += 1;
            }

            if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}