use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut values = input.lines().map(|s| s.trim().parse::<i32>().unwrap());
        let n = values.next().unwrap();
        let k = values.next().unwrap();
        let a = values.next().unwrap();
        let b = values.next().unwrap();

        let mut arr = [0; 200001];
        let mut aa = [0; 200001];
        for i in 1..=n {
            arr[i] = values.next().unwrap();
        }
        for i in 1..=n {
            aa[i] = values.next().unwrap();
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut a = a;
        let mut b = b;
        for _ in 1..=k {
            p1 = p1.max(1LL * aa[a] * (k - _ + 1) + p1);
            p1 += aa[a];
            a = arr[a];

            p2 = p2.max(1LL * aa[b] * (k - _ + 1) + p2);
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