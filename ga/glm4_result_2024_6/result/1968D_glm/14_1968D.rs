use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Please type a number");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut tokens = input.trim().split_whitespace();
        let n: i32 = tokens.next().unwrap().parse().expect("Please type a number");
        let k: i32 = tokens.next().unwrap().parse().expect("Please type a number");
        let a: i32 = tokens.next().unwrap().parse().expect("Please type a number");
        let b: i32 = tokens.next().unwrap().parse().expect("Please type a number");

        let mut arr = vec![0; (n + 1) as usize];
        let mut aa = vec![0; (n + 1) as usize];

        for i in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            arr[i as usize] = input.trim().parse().expect("Please type a number");
        }

        for i in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            aa[i as usize] = input.trim().parse().expect("Please type a number");
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut a = a;
        let mut b = b;

        for _ in 0..k {
            p1 = p1.max(1 * aa[a as usize]);
            a = arr[a as usize];
            p2 = p2.max(1 * aa[b as usize]);
            b = arr[b as usize];
        }

        if p1 > p2 {
            println!("Bodya");
        } else if p1 < p2 {
            println!("Sasha");
        } else {
            println!("Draw");
        }
    }
}