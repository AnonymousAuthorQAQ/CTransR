use std::io::{self, Read};
use std::time::SystemTime;

const N: usize = 200_000;

fn rand_() -> u32 {
    let mut seed = 1;
    seed = seed.wrapping_mul(3);
    seed >> 1
}

fn srand_() {
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as u32;
    unsafe {
        Z = seed ^ (seed << 1) | 1;
    }
}

static mut Z: u32 = 0;

fn sort(ii: &mut [usize], l: usize, r: usize) {
    if l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand_() as usize % (r - l)];
        while j < k {
            let c = if ii[j] != i_ { ii[j] - i_ } else { ii[j] - i_ };
            if c == 0 {
                j += 1;
            } else if c < 0 {
                ii.swap(i, j);
                i += 1;
                j += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        sort(ii, l, i);
        sort(ii, k, r);
    }
}

fn search(a: usize, i: usize, ii: &[usize]) -> usize {
    let mut lower = -1;
    let mut upper = ii.len() + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        if ii[h] < a || (ii[h] == a && ii[h] < i) {
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
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut lines = input.lines();
    let t = lines.next().unwrap().parse::<usize>().expect("Invalid t");

    for _ in 0..t {
        let mut line = lines.next().unwrap().split_whitespace();
        let n = line.next().unwrap().parse::<usize>().expect("Invalid n");
        let q = line.next().unwrap().parse::<usize>().expect("Invalid q");

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            aa[i] = line.next().unwrap().parse::<usize>().expect("Invalid aa[i]");
            aa[i] ^= aa[i - 1];
        }

        let mut ii = (0..=n).collect::<Vec<_>>();
        sort(&mut ii, 0, n + 1);

        for _ in 0..q {
            let mut line = lines.next().unwrap().split_whitespace();
            let l = line.next().unwrap().parse::<usize>().expect("Invalid l") - 1;
            let r = line.next().unwrap().parse::<usize>().expect("Invalid r");

            println!(
                "{}",
                if aa[l] == aa[r] || ii[search(aa[r], l, &ii) + 1] < ii[search(aa[l], r, &ii)] {
                    "YES"
                } else {
                    "NO"
                }
            );
        }
    }
}