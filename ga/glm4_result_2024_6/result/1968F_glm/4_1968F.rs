use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let n: usize = line.parse().unwrap();
        let q: usize = line.parse().unwrap();
        let mut aa = Vec::with_capacity(n + 1);
        aa.push(0);
        for _ in 1..=n {
            aa.push(lines.next().unwrap().unwrap().parse().unwrap());
        }
        let mut ii = (0..=n).collect::<Vec<usize>>();
        sort(&mut aa, &mut ii, 0, n + 1);
        for _ in 0..q {
            let mut line = lines.next().unwrap().unwrap();
            let l: usize = line.parse().unwrap() - 1;
            let r: usize = line.parse().unwrap();
            println!("{}", if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l) + 1] < ii[search(&aa, &ii, aa[l], r)] {
                "YES"
            } else {
                "NO"
            });
        }
    }
}

fn sort(aa: &mut Vec<usize>, ii: &mut Vec<usize>, l: usize, r: usize) {
    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand_(r - l)];
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
        l = k;
    }
}

fn rand_(n: usize) -> usize {
    static mut Z: usize = 0;
    if Z == 0 {
        let tv = unsafe {
            let mut tv: libc::timeval = std::mem::zeroed();
            libc::gettimeofday(&mut tv, std::ptr::null_mut());
            tv
        };
        Z = tv.tv_sec as usize ^ tv.tv_usec as usize | 1;
    }
    Z = Z.wrapping_mul(3) >> 1;
    Z % n
}

fn search(aa: &[usize], ii: &[usize], a: usize, i: usize) -> usize {
    let mut lower = -1;
    let mut upper = ii.len();
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