const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let mut n = 0;
    println!("Enter the number of elements: ");
    std::io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please type a number!");

    let mut x = [0; N];
    println!("Enter {} integers:", n - 1);
    for i in 1..n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        x[i] = input.trim().parse().expect("Please type a number!");
    }

    let mut a = [0; N];
    a[0] = x[1] + 1;
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n) && (x[i + 1] < a[i]) && (a[i] != a[i - 1]) {
                break;
            }
        }
    }

    println!("The resulting sequence is: ");
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut t = 0;
    println!("Enter the number of times to run the proc function: ");
    std::io::stdin().read_line(&mut t).expect("Failed to read line");
    let t: usize = t.trim().parse().expect("Please type a number!");

    for _ in 0..t {
        proc();
    }
}