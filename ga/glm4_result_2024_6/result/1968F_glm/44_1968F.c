use std::io::{self, BufRead};
use std::time::SystemTime;

const N: usize = 200000;

fn rand_() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = Z.wrapping_mul(3).wrapping_shr(1);
        Z
    }
}

fn srand_() {
    let now = SystemTime::now();
    let duration = now.elapsed().expect("Clock may have gone backwards");
    unsafe {
        Z = (duration.as_secs() as u32).wrapping_xor(duration.subsec_micros() as u32) | 1;
    }
}

fn sort(aa: &mut [i32], n: usize) {
    let mut ii = (0..=n).collect::<Vec<usize>>();
    let mut rng = || rand_() as usize % n;

    fn partition(aa: &mut [i32], ii: &mut [usize], l: usize, r: usize) -> usize {
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let mut i_ = ii[l + rng()];
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
        i
    }

    fn quick_sort(aa: &mut [i32], ii: &mut [usize], l: usize, r: usize) {
        if l < r {
            let p = partition(aa, ii, l, r);
            quick_sort(aa, ii, l, p);
            quick_sort(aa, ii, p + 1, r);
        }
    }

    quick_sort(aa, &mut ii, 0, n + 1);
    *aa = (0..=n).map(|i| aa[ii[i]]).collect::<Vec<i32>>();
}

fn search(a: i32, i: usize, aa: &[i32], ii: &[usize]) -> usize {
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

fn main() {
    srand_();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            aa[i] = iter.next().unwrap().parse().unwrap();
            aa[i] ^= aa[i - 1];
        }

        let mut ii = (0..=n).collect::<Vec<usize>>();
        sort(&mut aa, n);

        for _ in 0..q {
            let mut line = lines.next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap() - 1;
            let r: usize = iter.next().unwrap().parse().unwrap();

            println!(
                if aa[l] == aa[r] || ii[search(aa[r], l, &aa, &ii) + 1] < ii[search(aa[l], r, &aa, &ii)] {
                    "YES"
                } else {
                    "NO"
                }
            );
        }
    }
}