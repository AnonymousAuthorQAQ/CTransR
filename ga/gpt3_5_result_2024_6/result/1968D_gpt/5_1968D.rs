const N: usize = 200000;

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = parts[0];

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let arr: Vec<usize> = parts;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let aa: Vec<usize> = parts;

        let m = min(n, parts[1]);
        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;
        let mut i: i64 = -1;
        let mut i_: i64 = -1;
        let mut j: i64 = -1;
        let mut j_: i64 = -1;
        let mut a: usize = parts[2];
        let mut b: usize = parts[3];
        let mut a_: usize = a;
        let mut b_: usize = b;

        for x in 1..=m {
            p1 = maxl(p1, (aa[a - 1] as i64) * (parts[1] as i64 - x as i64 + 1) + p1_);
            p1_ += aa[a - 1] as i64;
            a = arr[a - 1];

            p2 = maxl(p2, (aa[b - 1] as i64) * (parts[1] as i64 - x as i64 + 1) + p2_);
            p2_ += aa[b - 1] as i64;
            b = arr[b - 1];
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