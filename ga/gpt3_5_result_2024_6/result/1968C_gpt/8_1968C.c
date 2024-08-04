use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();

    input.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    for _ in 0..t {
        input.read_line(&mut buffer).unwrap();
        let n: usize = buffer.trim().parse().unwrap();
        buffer.clear();

        let mut x: [usize; N] = [0; N];
        let mut a: [usize; N] = [0; N];

        input.read_line(&mut buffer).unwrap();
        let mut iter = buffer.split_ascii_whitespace().skip(1);
        for i in 1..n {
            x[i] = iter.next().unwrap().parse().unwrap();
        }
        buffer.clear();

        a[0] = x[1] + 1;
        for i in 1..n {
            for j in 0..=A {
                a[i] = x[i] + j * a[i - 1];
                if (i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1]) || i + 1 == n {
                    break;
                }
            }
        }

        for i in 0..n {
            print!("{} ", a[i]);
        }
        println!();
    }
}