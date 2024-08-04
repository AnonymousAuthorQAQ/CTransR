use std::io;

fn min(a: i32, b: i32) -> i32 {
    a.min(b)
}

fn maxl(a: i64, b: i64) -> i64 {
    a.max(b)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut values = input.trim().split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let n = values.next().unwrap();
        let k = values.next().unwrap();
        let a = values.next().unwrap();
        let b = values.next().unwrap();

        let mut arr = vec![0; n as usize + 1];
        let mut aa = vec![0; n as usize + 1];

        for i in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            arr[i as usize] = input.trim().parse().unwrap();
        }

        for i in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            aa[i as usize] = input.trim().parse().unwrap();
        }

        let m = min(n, k);
        let mut p1 = 0;
        let mut p2 = 0;
        let mut a = a;
        let mut b = b;

        for _ in 0..m {
            p1 = maxl(p1, (aa[a as usize] as i64) * (k as i64 - _ + 1) + p1);
            p1 += aa[a as usize];
            a = arr[a as usize];

            p2 = maxl(p2, (aa[b as usize] as i64) * (k as i64 - _ + 1) + p2);
            p2 += aa[b as usize];
            b = arr[b as usize];
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