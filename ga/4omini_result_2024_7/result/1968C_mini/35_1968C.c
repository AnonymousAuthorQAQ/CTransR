use std::io;

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc(n: usize, x: &[usize]) -> Vec<usize> {
    let mut a = vec![0; n];
    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (if i + 1 < n { x[i + 1] } else { 1 }) < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }
    a
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut x = vec![0; n];
        input.clear();
        stdin.read_line(&mut input).unwrap();
        
        let values: Vec<usize> = input.split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        
        for j in 1..n {
            x[j] = values[j - 1]; // Fill x from index 1 as in the original code
        }

        let a = proc(n, &x);
        println!("{}", a.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
    }
}