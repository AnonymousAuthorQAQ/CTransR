use std::io;
use std::time::SystemTime;

const N: usize = 200000;

fn rand() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = Z.wrapping_mul(3);
        Z >>= 1;
        Z
    }
}

fn srand() {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards");
    let usec = since_epoch.subsec_micros();
    let sec = since_epoch.as_secs();
    unsafe {
        Z = (sec as u32) ^ (usec as u32) | 1;
    }
}

fn sort(aa: &mut [i32], ii: &mut [usize], l: usize, r: usize) {
    if r - l > 1 {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand() as usize % (r - l)];
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
        sort(aa, ii, l, i);
        sort(aa, ii, k, r);
    }
}

fn search(a: i32, i: usize, aa: &[i32], ii: &[usize]) -> isize {
    let mut lower = -1;
    let mut upper = ii.len() as isize;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] < i) {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower
}

fn main() {
    srand();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.trim().split_whitespace();
        let n: usize = iter.next().expect("Invalid input").parse().expect("Invalid input");
        let q: usize = iter.next().expect("Invalid input").parse().expect("Invalid input");

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            aa[i] = iter.next().expect("Invalid input").parse().expect("Invalid input");
            aa[i] ^= aa[i - 1];
        }

        let mut ii = (0..=n).collect::<Vec<_>>();
        sort(&mut aa, &mut ii, 0, n + 1);

        for _ in 0..q {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let mut iter = input.trim().split_whitespace();
            let l: usize = iter.next().expect("Invalid input").parse().expect("Invalid input") - 1;
            let r: usize = iter.next().expect("Invalid input").parse().expect("Invalid input") - 1;

            println!(
                "{}",
                if aa[l] == aa[r] || search(aa[r], l, &aa, &ii) + 1 < search(aa[l], r, &aa, &ii) as usize {
                    "YES"
                } else {
                    "NO"
                }
            );
        }
    }
}