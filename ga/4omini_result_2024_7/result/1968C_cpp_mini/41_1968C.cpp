use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap(); // Number of test cases
    
    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        
        let n: usize = input.trim().parse().unwrap(); // Number of elements
        let mut x = vec![0; n];
        let mut a = vec![0; n];
        
        // Reading x[1] to x[n-1]
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let x_values: Vec<i32> = input.trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        for i in 1..n {
            x[i] = x_values[i - 1];
        }
        
        a[0] = x[1] + 1; // a[0] is defined as x[1] + 1

        for i in 1..n {
            for j in 0..=1000000000 { // A is effectively a large constant
                a[i] = x[i] + j * a[i - 1];
                if (if i + 1 < n { x[i + 1] } else { 1 }) < a[i] && a[i] != a[i - 1] {
                    break;
                }
            }
        }
        
        // Print the results for this test case
        println!("{}", a.iter().map(|&val| val.to_string()).collect::<Vec<String>>().join(" "));
    }
}