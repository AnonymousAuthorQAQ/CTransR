use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t = lines.next().unwrap().parse::<i32>().unwrap();
    for _ in 0..t {
        let mut p1 = 0;
        let mut p1_prime = 0;
        let mut p2 = 0;
        let mut p2_prime = 0;
        let mut a = 0;
        let mut b = 0;
        let mut arr = [0; 200001];
        let mut aa = [0; 200001];

        let nkaab = lines.next().unwrap().split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let (n, k, a, b) = (nkaab.next().unwrap(), nkaab.next().unwrap(), nkaab.next().unwrap(), nkaab.next().unwrap());
        let m = n.min(k);

        for i in 1..=n {
            arr[i] = lines.next().unwrap().parse::<i32>().unwrap();
        }

        for i in 1..=n {
            aa[i] = lines.next().unwrap().parse::<i32>().unwrap();
        }

        for _ in 1..=m {
            p1 = p1.max(1LL * aa[a] * (k - a + 1) + p1_prime);
            p1_prime += aa[a];
            a = arr[a];

            p2 = p2.max(1LL * aa[b] * (k - b + 1) + p2_prime);
            p2_prime += aa[b];
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