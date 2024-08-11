use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();

    while t > 0 {
        let n: usize = lines.next().unwrap().parse().unwrap();
        let mut x = [0; 501];
        for i in 1..n {
            x[i] = lines.next().unwrap().parse().unwrap();
        }
        let mut a = [0; 501];
        a[0] = x[1] + 1;
        for i in 1..n {
            for j in 0..1000000001 {
                a[i] = x[i] + j * a[i - 1];
                if i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1] {
                    break;
                }
            }
        }
        for i in 0..n {
            print!("{} ", a[i]);
        }
        println!();
        t -= 1;
    }
}