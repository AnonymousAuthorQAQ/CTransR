const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let mut x = [0; N];
    let mut a = [0; N];

    let n = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<usize>().unwrap()
    };

    for i in 1..n {
        x[i] = {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.trim().parse::<i32>().unwrap()
        };
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (i + 1 == n && 1 < a[i]) {
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
    let t = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<i32>().unwrap()
    };

    for _ in 0..t {
        proc();
    }
}