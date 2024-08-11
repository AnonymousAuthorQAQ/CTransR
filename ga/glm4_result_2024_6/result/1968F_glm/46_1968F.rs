use std::io::{self, BufRead};
use std::time::SystemTime;

const N: usize = 200000;

fn rand() -> u32 {
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u32;
    let mut z = time ^ (time << 1) | 1;
    z = z.wrapping_mul(3);
    z >> 1
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    if r - l > 1 {
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let i_ = ii[l + rand() as usize % (r - l)];
        let mut tmp;

        while j < k {
            let c = if i_ < j { i_ as i32 - j as i32 } else { (ii[j] as i32 - ii[i_] as i32).unsigned_abs() };
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

fn search(a: u32, i: usize, aa: &[u32], ii: &[usize]) -> usize {
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
    lower as usize
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    while t > 0 {
        let mut input = lines.next().unwrap().unwrap().split_whitespace();
        let n = input.next().unwrap().parse::<usize>().unwrap();
        let q = input.next().unwrap().parse::<usize>().unwrap();

        let mut aa: Vec<u32> = Vec::with_capacity(n + 1);
        let mut ii: Vec<usize> = (0..=n).collect();

        for _ in 0..n {
            aa.push(input.next().unwrap().parse::<u32>().unwrap());
        }

        sort(&mut ii, 0, n + 1);

        for _ in 0..q {
            let mut input = lines.next().unwrap().unwrap().split_whitespace();
            let l = input.next().unwrap().parse::<usize>().unwrap() - 1;
            let r = input.next().unwrap().parse::<usize>().unwrap();

            println!("{}", if aa[l] <= aa[r] || ii[search(aa[r], l + 1, &aa, &ii) + 1] < ii[search(aa[l], r + 1, &aa, &ii)] {
                "YES"
            } else {
                "NO"
            });
        }

        t -= 1;
    }
}