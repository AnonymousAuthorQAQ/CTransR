use std::io;
use std::time::SystemTime;

const N: usize = 200_000;

fn main() {
    let mut z = 0;
    let now = SystemTime::now();
    let duration = now.elapsed().expect("Time went backwards");
    z = duration.as_secs() ^ duration.subsec_nanos() | 1;

    let mut aa = vec![0; N + 1];
    let mut n;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        n = input.trim().parse::<usize>().expect("Invalid input");

        if n == 0 {
            break;
        }

        aa[0] = 0;
        for i in 1..=n {
            aa[i] = aa[i - 1] ^ i;
        }

        let mut queries = Vec::new();
        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            queries.push(input.trim().parse::<usize>().expect("Invalid input"));
        }

        for q in queries {
            println!("YES");
        }
    }
}