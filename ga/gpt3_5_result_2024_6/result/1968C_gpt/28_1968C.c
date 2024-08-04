const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let mut x: [i32; N] = [0; N];
    let mut a: [i32; N] = [0; N];

    let mut n = 0;
    if let Ok(num) = std::io::stdin().read_line(&mut n) {
        if num == 0 {
            return;
        }
    }

    let mut input_line = String::new();
    std::io::stdin().read_line(&mut input_line).unwrap();
    let mut iter = input_line.split_whitespace();

    for i in 1..n {
        x[i] = iter.next().unwrap().parse().unwrap();
    }

    a[0] = x[1] + 1;
    
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            
            if (i + 1 < n && x[i + 1] < a[i]) || a[i] != a[i - 1] {
                break;
            }
            j += 1;
        }
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: i32 = t.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}