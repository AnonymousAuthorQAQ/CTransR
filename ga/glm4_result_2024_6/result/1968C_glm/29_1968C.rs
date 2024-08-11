const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let mut n;
    loop {
        n = 0;
        if let Err(e) = std::io::stdin().read_line(&mut n) {
            eprintln!("Failed to read line: {}", e);
            break;
        }
        let n: usize = n.trim().parse().expect("Please enter a valid number");
        if n >= N {
            println!("Number too large. Please enter a number less than {}", N);
            continue;
        }
        let mut x = [0; N];
        for i in 1..n {
            loop {
                let mut line = String::new();
                if let Err(e) = std::io::stdin().read_line(&mut line) {
                    eprintln!("Failed to read line: {}", e);
                    break;
                }
                if let Ok(x_val) = line.trim().parse::<i32>() {
                    x[i] = x_val;
                    break;
                } else {
                    println!("Please enter a valid number");
                }
            }
        }
        let mut a = [0; N];
        a[0] = x[1] + 1;
        for i in 1..n {
            for j in 0..=A {
                a[i] = x[i] + j * a[i - 1];
                if (i + 1 < n && x[i + 1] < a[i]) || (i + 1 == n && 1 < a[i]) {
                    break;
                }
            }
        }
        for &ai in &a {
            print!("{} ", ai);
        }
        println!();
    }
}

fn main() {
    let mut t = 0;
    loop {
        println!("Enter the number of test cases: ");
        let mut line = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut line) {
            eprintln!("Failed to read line: {}", e);
            break;
        }
        if let Ok(t_val) = line.trim().parse::<i32>() {
            t = t_val;
            break;
        } else {
            println!("Please enter a valid number");
        }
    }
    for _ in 0..t {
        proc();
    }
}