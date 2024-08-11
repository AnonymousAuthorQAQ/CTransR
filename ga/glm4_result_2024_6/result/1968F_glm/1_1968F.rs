use std::io;
use std::time::SystemTime;

const N: usize = 200_000;

fn rand_() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = (Z.wrapping_mul(3)).wrapping_shr(1);
        Z
    }
}

fn srand_() {
    let now = SystemTime::now();
    let duration = now.elapsed().expect("Time went backwards");
    let mut tv = (duration.as_secs() as u32, duration.subsec_nanos() as u32);
    unsafe {
        Z = tv.0 ^ tv.1 | 1;
    }
}

fn sort(aa: &mut [i32], ii: &mut [usize]) {
    if aa.len() <= 1 {
        return;
    }
    let mut i = 0;
    let mut j = 0;
    let mut k = aa.len();
    let i_ = ii[rand_() as usize % (k - i)];
    let mut tmp;

    while j < k {
        let c = if aa[ii[j]] != aa[i_] {
            aa[ii[j]] - aa[i_]
        } else {
            ii[j] - i_
        };
        if c == 0 {
            j += 1;
        } else if c < 0 {
            tmp = ii[i];
            ii[i] = ii[j];
            ii[j] = tmp;
            i += 1;
            j += 1;
        } else {
            k -= 1;
            tmp = ii[j];
            ii[j] = ii[k];
            ii[k] = tmp;
        }
    }

    sort(&mut aa[i..k], &mut ii[i..k]);
}

fn search(a: i32, i: usize, aa: &[i32], ii: &[usize]) -> usize {
    let mut lower = -1;
    let mut upper = aa.len() + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if aa[ii[h]] < a || (aa[ii[h]] == a && ii[h] < i) {
            lower = h;
        } else {
            upper = h;
        }
    }

    lower
}

fn main() {
    srand_();

    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).expect("Failed to read line");
    let t: usize = line.trim().parse().expect("Invalid input");

    for _ in 0..t {
        line.clear();
        stdin.read_line(&mut line).expect("Failed to read line");
        let mut tokens = line.trim().split_whitespace();
        let n: usize = tokens.next().expect("Invalid input").parse().expect("Invalid input");
        let q: usize = tokens.next().expect("Invalid input").parse().expect("Invalid input");

        let mut aa = vec![0; n + 1];
        let mut ii = (0..=n).collect::<Vec<usize>>();

        for i in 1..=n {
            line.clear();
            stdin.read_line(&mut line).expect("Failed to read line");
            aa[i] = line.trim().parse().expect("Invalid input");
            aa[i] ^= aa[i - 1];
        }

        sort(&mut aa, &mut ii);

        for _ in 0..q {
            line.clear();
            stdin.read_line(&mut line).expect("Failed to read line");
            let mut tokens = line.trim().split_whitespace();
            let l: usize = tokens.next().expect("Invalid input").parse().expect("Invalid input") - 1;
            let r: usize = tokens.next().expect("Invalid input").parse().expect("Invalid input");

            let a = aa[l];
            let i = ii[search(a, l, &aa, &ii) + 1];

            println!("{}", if aa[l] == aa[r] || i < ii[search(aa[r], r, &aa, &ii)] {
                "YES"
            } else {
                "NO"
            });
        }
    }
}