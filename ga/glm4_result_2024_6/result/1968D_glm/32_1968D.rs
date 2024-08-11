use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t = lines.next().unwrap().parse::<i32>().unwrap();

    while t > 0 {
        let mut n, k, a, b = 0;
        let mut arr = [0; 200001];
        let mut aa = [0; 200001];

        let mut parts = lines.next().unwrap().split_whitespace();
        n = parts.next().unwrap().parse::<i32>().unwrap();
        k = parts.next().unwrap().parse::<i32>().unwrap();
        a = parts.next().unwrap().parse::<i32>().unwrap();
        b = parts.next().unwrap().parse::<i32>().unwrap();

        for i in 1..=n {
            arr[i] = lines.next().unwrap().parse::<i32>().unwrap();
        }

        for i in 1..=n {
            aa[i] = lines.next().unwrap().parse::<i32>().unwrap();
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut a = a;
        let mut b = b;

        for _ in 1..=k {
            p1 = p1.max(aa[a] * (k as i32 - _ + 1) + p1);
            p1 += aa[a];
            a = arr[a];

            p2 = p2.max(aa[b] * (k as i32 - _ + 1) + p2);
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

        t -= 1;
    }
}