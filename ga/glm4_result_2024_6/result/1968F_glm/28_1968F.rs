use std::io;
use std::time::SystemTime;

const N: usize = 200000;

fn rand_() -> u32 {
    unsafe {
        static mut Z: u32 = 0;
        Z = Z.wrapping_mul(3).wrapping_shr(1);
        Z
    }
}

fn srand_() {
    let now = SystemTime::now();
    let duration = now.elapsed().unwrap();
    let tv_sec = duration.as_secs() as u32;
    let tv_usec = duration.subsec_micros() as u32;
    unsafe {
        static mut Z: u32 = 0;
        Z = tv_sec ^ tv_usec | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    if l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand_() as usize % (r - l)];
        while j < k {
            let c = if i_ == j { i_ - j } else { i_ - j };
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

fn search(a: u32, i: usize, aa: &[u32], ii: &[usize]) -> usize {
    let mut lower = -1;
    let mut upper = ii.len() + 1;

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

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        let mut aa = vec![0; n + 1];
        let mut ii = (0..=n).collect::<Vec<usize>>();

        for i in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let a: u32 = input.trim().parse().unwrap();
            aa[i] = a ^ aa[i - 1];
        }

        sort(&mut ii, 0, n + 1);

        for _ in 0..q {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut parts = input.trim().split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap() - 1;
            let r: usize = parts.next().unwrap().parse().unwrap();

            let result = if aa[l] == aa[r] || ii[search(aa[r], l, &aa, &ii) + 1] < ii[search(aa[l], r, &aa, &ii)] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}