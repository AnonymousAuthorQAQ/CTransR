use std::io::{self, BufRead};
use std::io::Write;

const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc(x: &Vec<i32>) -> Vec<i32> {
    let n = x.len();
    let mut a = vec![0; n];
    let mut i = 1;
    let mut j: i32;

    a[0] = x[1] + 1;

    while i < n {
        j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if i + 1 < n {
                break;
            } else if x[i + 1] < a[i] && a[i] != a[i - 1] {
                break;
            }
            j += 1;
        }
        i += 1;
    }

    a
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.lock().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let x: Vec<i32> = input.split_whitespace().map(|num| num.parse().unwrap()).collect();
        let result = proc(&x[1..].to_vec());

        let result_str: String = result.iter().map(|num| num.to_string() + " ").collect();
        println!("{}", result_str);
    }
}