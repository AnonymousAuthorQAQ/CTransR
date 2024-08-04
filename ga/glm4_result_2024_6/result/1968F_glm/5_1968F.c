use std::io;
use std::time::SystemTime;

const N: usize = 200_000;

fn rand_() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = (Z * 3) >> 1;
        Z
    }
}

fn srand_() {
    let tv = SystemTime::now();
    let duration = tv.elapsed().expect("SystemTime::elapsed failed");
    unsafe {
        Z = (duration.as_secs() as u32) ^ (duration.subsec_nanos() as u32) | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    if l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand_() as usize % (r - l)];
        let mut tmp;

        while j < k {
            let c = if ii[j] != i_ {
                if ii[j] < i_ { -1 } else { 1 }
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

        sort(ii, l, i);
        sort(ii, k, r);
    }
}

fn search(a: i32, i: usize, aa: &[i32], ii: &[usize]) -> isize {
    let mut lower = -1;
    let mut upper = ii.len() as isize;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] < i as usize) {
            lower = h;
        } else {
            upper = h;
        }
    }

    lower
}

fn main() {
    srand_();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t = input.trim().parse::<usize>().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().expect("Invalid input");
        let q: usize = parts.next().unwrap().parse().expect("Invalid input");

        let mut aa = vec![0; n + 1];
        let mut ii = (0..=n).collect::<Vec<usize>>();

        for i in 1..=n {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            aa[i] = input.trim().parse::<i32>().expect("Invalid input");
            aa[i] ^= aa[i - 1];
        }

        sort(&mut ii, 0, n + 1);

        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let mut parts = input.trim().split_whitespace();
            let l: usize = parts.next().unwrap().parse().expect("Invalid input") - 1;
            let r: usize = parts.next().unwrap().parse().expect("Invalid input");

            println!(
                "{}",
                if aa[l] == aa[r] || search(aa[r], l, &aa, &ii) + 1 < search(aa[l], r, &aa, &ii) {
                    "YES"
                } else {
                    "NO"
                }
            );
        }
    }
}