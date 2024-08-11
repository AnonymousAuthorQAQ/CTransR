use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let t = input.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let n = tokens.next().unwrap();
        let k = tokens.next().unwrap();
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();

        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];

        for x in 1..=n {
            arr[x] = tokens.next().unwrap();
        }

        for x in 1..=n {
            aa[x] = tokens.next().unwrap();
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut a = a;
        let mut b = b;
        for x in 1..=k {
            p1 = p1.max(1LL * aa[a] * (k - x + 1));
            p1 += aa[a];
            a = arr[a];

            p2 = p2.max(1LL * aa[b] * (k - x + 1));
            p2 += aa[b];
            b = arr[b];
        }

        if p1 > p2 {
            println!("Bodya");
        } else if p1 < p2 {
            println!("Sasha");
        } else {
            println!("Draw");
        }
    }
}