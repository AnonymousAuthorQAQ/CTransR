use std::io::{self, Read};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc(x: &[usize], n: usize) -> Vec<usize> {
    let mut a = vec![0; n];
    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (a[i] != a[i - 1]) {
                break;
            }
        }
    }
   
    a
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let mut x = vec![0; n];
        
        for i in 1..n {
            x[i] = iter.next().unwrap().parse().unwrap();
        }
        
        let result = proc(&x, n);
        println!("{}", result.iter().map(|&val| val.to_string()).collect::<Vec<_>>().join(" "));
    }
}