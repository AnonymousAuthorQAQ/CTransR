use std::io;

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc(n: usize, x: &[usize]) {
    let mut a = vec![0; N];

    a[0] = x[1] + 1;
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n) as usize + 1 < n && (x[i + 1] < a[i]) && (a[i] != a[i - 1]) {
                break;
            }
        }
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();
        let mut x = vec![0; N];

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<usize> = input.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        for (i, &num) in nums.iter().enumerate() {
            x[i + 1] = num; // Fill x starting from index 1
        }

        proc(n, &x);
    }
}