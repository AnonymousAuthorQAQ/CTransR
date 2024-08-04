use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i64 = 1000000000;

fn proc(x: &Vec<i64>, n: usize) -> Vec<i64> {
    let mut a = vec![0; n];
    a[0] = x[1] + 1;
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (i + 1 >= n && a[i] != a[i - 1]) {
                break;
            }
            j += 1;
        }
    }
    a
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut buffer = String::new();
    input.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let n: usize = buffer.trim().parse().unwrap();
        
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let x: Vec<i64> = buffer.split_whitespace().map(|s| s.parse().unwrap()).collect();

        let result = proc(&x, n);
        println!("{}", result.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}