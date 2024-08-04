use std::io::{self, BufRead};

const N: usize = 500;
const A: i32 = 1_000_000_000;

fn proc(n: usize, x: &[i32]) -> Vec<i32> {
    let mut a = vec![0; n];
    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n) as i32 * x[i + 1] < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }
    a
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut line_index = 0;
    let t: usize = lines[line_index].parse().unwrap();
    line_index += 1;

    for _ in 0..t {
        let n: usize = lines[line_index].parse().unwrap();
        line_index += 1;
        
        let mut x = vec![0; n];
        for i in 1..n {
            x[i] = lines[line_index].parse().unwrap();
        }
        line_index += 1;

        let a = proc(n, &x);
        println!("{}", a.iter().map(|&val| val.to_string()).collect::<Vec<String>>().join(" "));
    }
}