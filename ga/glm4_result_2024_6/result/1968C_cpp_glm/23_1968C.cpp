const N: usize = 500;
const X: usize = 500;
const A: i32 = 100_000_000;

fn proc() {
    let mut n;
    loop {
        println!("Enter n:");
        n = read_int();
        if n < N {
            break;
        }
        println!("n must be less than {}", N);
    }

    let mut x = vec![0; n];
    println!("Enter {} integers:", n-1);
    for i in 1..n {
        x[i] = read_int();
    }

    let mut a = vec![0; n];
    a[0] = x[1] + 1;
    for i in 1..n {
        let mut j = 0;
        while j <= A as usize {
            a[i] = x[i] + j as i32 * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (i + 1 >= n && 1 < a[i]) {
                break;
            }
            j += 1;
        }
    }

    println!("Output:");
    for i in 0..n {
        println!("{}", a[i]);
    }
}

fn read_int() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let mut t;
    loop {
        println!("Enter t:");
        t = read_int();
        if t >= 0 {
            break;
        }
        println!("t must be non-negative");
    }

    for _ in 0..t {
        proc();
    }
}