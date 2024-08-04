const N: usize = 500;
const X: usize = 500;
const A: i64 = 1000000000;

fn proc() {
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    let mut n = 0;
    let mut input_line = String::new();
    std::io::stdin().read_line(&mut input_line).unwrap();
    let mut nums = input_line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap());
    n = nums.next().unwrap();

    for i in 1..n {
        let x_value: i64 = nums.next().unwrap();
        x[i] = x_value;
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

    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut input_line = String::new();
    std::io::stdin().read_line(&mut input_line).unwrap();
    let t: i64 = input_line.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}