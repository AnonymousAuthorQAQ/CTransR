const N: usize = 500;
const X: usize = 500;
const A: i64 = 1000000000;

fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let n: usize = input.trim().parse().unwrap();
        
        let mut x: Vec<i64> = vec![0; n];
        let mut a: Vec<i64> = vec![0; n];
        input = String::new();
        
        std::io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        
        for i in 1..n {
            x[i] = parts.next().unwrap().parse().unwrap();
        }
        
        a[0] = x[1] + 1;
        
        for i in 1..n {
            let mut j = 0;
            while j <= A {
                a[i] = x[i] + j * a[i - 1];
                if (i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1]) {
                    break;
                }
                j += 1;
            }
        }
        
        for val in a.iter() {
            print!("{} ", val);
        }
        println!();
    }
}