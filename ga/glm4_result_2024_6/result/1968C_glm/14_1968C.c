const N: usize = 500;
const X: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let mut n = 0;
    let mut x: Vec<i32> = Vec::with_capacity(N);
    let mut a: Vec<i32> = Vec::with_capacity(N);

    // Read number of elements
    n = read_int();

    // Read sequence
    for _ in 1..n {
        x.push(read_int());
    }

    // Initialize a[0]
    a.push(x[1] + 1);

    // Compute the sequence a
    for i in 1..n {
        let mut j = 0;
        loop {
            a.push(x[i] + j * a[i - 1]);
            if i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1] {
                break;
            }
            j += 1;
            if j > A {
                break;
            }
        }
    }

    // Print the sequence a
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn read_int() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: i32 = input.trim().parse().expect("Please enter a number");
    num
}

fn main() {
    let mut t = 0;
    // Read number of test cases
    t = read_int() as usize;
    // Run the proc function t times
    for _ in 0..t {
        proc();
    }
}