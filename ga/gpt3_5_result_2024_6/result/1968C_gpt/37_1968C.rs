const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().expect("Invalid input");

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let x: Vec<i32> = input.trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let mut a: Vec<i32> = vec![0; n];
        a[0] = x[1] + 1;
        
        for i in 1..n {
            for j in 0..=A {
                a[i] = x[i] + j * a[i - 1];
                
                if (i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1]) || (i + 1 >= n) {
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

fn main() {
    proc();
}