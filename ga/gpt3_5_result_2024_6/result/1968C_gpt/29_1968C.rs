const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let mut x = [0; N];
    let mut a = [0; N];
    
    let mut n = 0;
    if let Ok(num) = read_line().trim().parse::<usize>() {
        n = num;
    }

    if n == 0 {
        return;
    }

    let mut input_arr = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input_arr) {
        let mut iter = input_arr.split_whitespace();
        for i in 1..n {
            if let Some(num) = iter.next() {
                if let Ok(num_val) = num.parse::<usize>() {
                    x[i] = num_val;
                }
            }
        }
    }

    a[0] = x[1] + 1;
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] || i + 1 == n) && a[i] != a[i - 1] {
                break;
            }
        }
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn main() {
    let mut input_line = String::new();
    if let Ok(t) = read_line().trim().parse::<usize>() {
        for _ in 0..t {
            proc();
        }
    }
}