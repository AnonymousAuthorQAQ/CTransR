fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();

        let n: i32 = iter.next().unwrap().parse().unwrap();
        let k: i32 = iter.next().unwrap().parse().unwrap();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();

        let m = std::cmp::min(n, k);
        let mut arr: Vec<i32> = Vec::with_capacity(n as usize);
        let mut aa: Vec<i32> = Vec::with_capacity(n as usize);

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        for _ in 0..n {
            arr.push(iter.next().unwrap().parse().unwrap());
        }

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        for _ in 0..n {
            aa.push(iter.next().unwrap().parse().unwrap());
        }

        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;
        let mut a_: i32 = a;
        let mut b_: i32 = b;

        for x in 1..=m {
            p1 = p1.max(1i64 * aa[a as usize] * (k - x + 1) + p1_);
            p1_ += aa[a as usize];
            a = arr[a as usize];

            p2 = p2.max(1i64 * aa[b as usize] * (k - x + 1) + p2_);
            p2_ += aa[b as usize];
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