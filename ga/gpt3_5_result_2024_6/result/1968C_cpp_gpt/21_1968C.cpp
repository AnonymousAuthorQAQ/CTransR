use std::io::{self, BufRead};

const N: usize = 500;
const A: i32 = 1000000000;

fn proc(x: &[i32], n: usize) -> Vec<i32> {
    let mut a = vec![0; n];
    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1]) || i + 1 == n {
                break;
            }
        }
    }
    a
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut input_line = String::new();
    input.read_line(&mut input_line).unwrap();
    let t: usize = input_line.trim().parse().unwrap();

    for _ in 0..t {
        input_line.clear();
        input.read_line(&mut input_line).unwrap();
        let n: usize = input_line.trim().parse().unwrap();

        input_line.clear();
        input.read_line(&mut input_line).unwrap();
        let x: Vec<i32> = input_line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        let result = proc(&x, n);
        for val in result {
            print!("{} ", val);
        }
        println!();
    }
}