use std::io;
use std::time::SystemTime;

const N: usize = 200000;

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
    unsafe {
        Z = (duration.as_secs() as u32) ^ (duration.subsec_nanos() as u32) | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand_() as usize % (r - l)];
        let mut tmp;

        while j < k {
            let c = if super::aa[ii[j]] != super::aa[i_] {
                super::aa[ii[j]] - super::aa[i_]
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
        l = k;
    }
}

fn search(a: u32, i: usize) -> isize {
    let mut lower = -1;
    let mut upper = super::n + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if super::aa[super::ii[h]] < a || (super::aa[super::ii[h]] == a && super::ii[h] < i) {
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
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut values = input.trim().split_whitespace();
        super::n = values.next().unwrap().parse().expect("Invalid input");
        let q: usize = values.next().unwrap().parse().expect("Invalid input");

        for i in 1..=super::n {
            super::aa[i] = values.next().unwrap().parse().expect("Invalid input");
            super::aa[i] ^= super::aa[i - 1];
        }

        for i in 0..=super::n {
            super::ii[i] = i;
        }
        sort(&mut super::ii, 0, super::n + 1);

        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let mut values = input.trim().split_whitespace();
            let l: usize = values.next().unwrap().parse().expect("Invalid input") - 1;
            let r: usize = values.next().unwrap().parse().expect("Invalid input");

            if super::aa[l] == super::aa[r] || super::ii[search(super::aa[r], l) as usize + 1] < super::ii[search(super::aa[l], r)] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}