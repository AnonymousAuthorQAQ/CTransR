const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let mut x = [0; N];
    let mut a = [0; N];

    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };

    for i in 1..n {
        x[i] = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<usize>().unwrap()
        };
    }

    a[0] = x[1] + 1;
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (i + 1 == n && 1 < a[i]) {
                break;
            }
        }
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let t = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };

    for _ in 0..t {
        proc();
    }
}