const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let mut n = 0;
    let mut x = [0; N];
    let mut a = [0; N];

    // Read n
    n = read_int();

    // Read x array
    for i in 1..n {
        x[i] = read_int();
    }

    // Initialize a[0]
    a[0] = x[1] + 1;

    // Calculate a[i] for each i
    for i in 1..n {
        let mut j = 0;
        while j <= A as usize {
            a[i] = x[i] + j as i32 * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (a[i] != a[i - 1]) {
                break;
            }
            j += 1;
        }
    }

    // Print a array
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn read_int() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let mut t = 0;
    t = read_int();

    while t > 0 {
        proc();
        t -= 1;
    }
}