use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc(x: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut a: Vec<i32> = vec![0; N];
    
    a[0] = x[1] + 1; // Starting from x[1] because x[0] is not read

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + (j as i32) * a[i - 1];

            if i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }
    
    a.into_iter().take(n).collect()
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines().filter_map(Result::ok);
    
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = lines.next().unwrap().parse().unwrap();
        let mut x: Vec<i32> = vec![0; N];

        for i in 1..n {
            x[i] = lines.next().unwrap().parse().unwrap();
        }

        let result = proc(&x, n);
        println!("{}", result.iter().take(n).map(|&num| num.to_string()).collect::<Vec<String>>().join(" "));
    }
}