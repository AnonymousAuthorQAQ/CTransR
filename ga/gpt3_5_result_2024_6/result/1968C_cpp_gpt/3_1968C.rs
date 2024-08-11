use std::io;

const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut x: Vec<usize> = vec![0; n];
        let mut a: Vec<usize> = vec![0; n];

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();

        for i in 1..n {
            x[i] = iter.next().unwrap().parse().unwrap();
        }

        a[0] = x[1] + 1;
        for i in 1..n {
            let mut j = 0;
            while j <= A {
                a[i] = x[i] + j * a[i - 1];
                if (i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1]) || i + 1 == n {
                    break;
                }
                j += 1;
            }
        }

        for i in 0..n {
            print!("{} ", a[i]);
        }
        println!();
    }
}