use std::io;
use std::time::SystemTime;

const N: usize = 200_000;

fn rand_() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        Z
    }
}

fn srand_() {
    let now = SystemTime::now();
    let duration = now.elapsed().unwrap();
    let (sec, usec) = (duration.as_secs(), duration.subsec_micros());
    unsafe {
        Z = (sec as u32) ^ (usec as u32) | 1;
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
            let c = if ii[j] < i_ { -1 } else if ii[j] == i_ && j < i_ { 1 } else { 0 };

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

fn search(a: u32, i: usize, aa: &[u32], ii: &[usize]) -> isize {
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
    srand_();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<usize>().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n = parts.next().unwrap().parse::<usize>().unwrap();
        let q = parts.next().unwrap().parse::<usize>().unwrap();

        let mut aa = vec![0; n + 1];
        let mut ii = (0..=n).collect::<Vec<usize>>();

        for i in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            aa[i] = input.trim().parse::<u32>().unwrap();
            aa[i] ^= aa[i - 1];
        }

        sort(&mut ii, 0, n + 1);

        for _ in 0..q {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut parts = input.trim().split_whitespace();
            let l = parts.next().unwrap().parse::<usize>().unwrap() - 1;
            let r = parts.next().unwrap().parse::<usize>().unwrap();

            let a = aa[l];
            let i = l + 1;

            let lower = search(a, i, &aa, &ii);

            if lower >= 0 && (aa[r] == a || (lower < (n as isize) && ii[lower as usize] < ii[search(aa[r], r + 1, &aa, &ii) as usize])) {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}