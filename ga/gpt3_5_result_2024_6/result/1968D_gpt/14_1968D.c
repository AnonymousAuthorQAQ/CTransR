use std::cmp;

const N: usize = 200000;

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let mut t;
    let mut x;
    let mut n;
    let mut k;
    let mut a;
    let mut b;
    let mut a_;
    let mut b_;
    let mut m;
    let mut arr: [i32; N + 1] = [0; N + 1];
    let mut aa: [i32; N + 1] = [0; N + 1];

    let mut p1;
    let mut p2;
    let mut p1_;
    let mut p2_;

    let mut input_line = String::new();
    std::io::stdin().read_line(&mut input_line).unwrap();
    t = input_line.trim().parse().unwrap();

    while t > 0 {
        p1 = 0;
        p2 = 0;
        p1_ = 0;
        p2_ = 0;
        let mut i = -1;
        let mut i_ = -1;
        let mut j = -1;
        let mut j_ = -1;

        input_line.clear();
        std::io::stdin().read_line(&mut input_line).unwrap();
        let input_str: Vec<i32> = input_line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        n = input_str[0];
        k = input_str[1];
        a = input_str[2];
        b = input_str[3];
        m = cmp::min(n, k);
        a_ = a;
        b_ = b;

        input_line.clear();
        std::io::stdin().read_line(&mut input_line).unwrap();
        let arr_input: Vec<i32> = input_line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for i in 1..=n {
            arr[i] = arr_input[i - 1];
        }

        input_line.clear();
        std::io::stdin().read_line(&mut input_line).unwrap();
        let aa_input: Vec<i32> = input_line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for i in 1..=n {
            aa[i] = aa_input[i - 1];
        }

        x = 1;
        while x <= m {
            p1 = maxl(p1, 1 * aa[a as usize] * (k - x + 1) + p1_);
            p1_ += aa[a as usize];
            a = arr[a as usize];
            p2 = maxl(p2, 1 * aa[b as usize] * (k - x + 1) + p2_);
            p2_ += aa[b as usize];
            b = arr[b as usize];

            x += 1;
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