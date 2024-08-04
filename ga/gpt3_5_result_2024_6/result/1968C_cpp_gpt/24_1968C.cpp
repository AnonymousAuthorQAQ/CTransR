use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;
static mut X: [usize; N] = [0; N];
static mut A: [usize; N] = [0; N];

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    stdin.lock().read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        stdin.lock().read_line(&mut buffer).unwrap();
        let n: usize = buffer.trim().parse().unwrap();

        buffer.clear();
        stdin.lock().read_line(&mut buffer).unwrap();
        let numbers: Vec<usize> = buffer
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        unsafe {
            for i in 1..n {
                X[i] = numbers[i - 1];
            }
            A[0] = X[1] + 1;
            for i in 1..n {
                let mut j = 0;
                while j <= A {
                    A[i] = X[i] + j * A[i - 1];
                    if i + 1 < n {
                        if X[i + 1] < A[i] && A[i] != A[i - 1] {
                            break;
                        }
                    } else {
                        break;
                    }
                    j += 1;
                }
            }
            for i in 0..n {
                print!("{} ", A[i]);
            }
            println!();
        }
    }
}