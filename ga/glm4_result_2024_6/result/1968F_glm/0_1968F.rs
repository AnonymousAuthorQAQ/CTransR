use std::io::{self, BufRead};

const N: usize = 200000;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let nq: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let n = nq[0];
        let q = nq[1];

        let mut aa: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let mut ii: Vec<usize> = (0..=n).collect();

        sort(&mut aa, &mut ii, 0, n + 1);

        for _ in 0..q {
            let lr: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
            let l = lr[0] - 1;
            let r = lr[1] - 1;

            println!("{}", if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l) + 1] < ii[search(&aa, &ii, aa[l], r)] {
                "YES"
            } else {
                "NO"
            });
        }
    }
}

fn sort(aa: &Vec<i32>, ii: &mut Vec<usize>, l: usize, r: usize) {
    if l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand_(r - l)];
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] { aa[ii[j]] - aa[i_] } else { ii[j] - i_ };

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

fn rand_(range: usize) -> usize {
    let mut z: u64 = 0;
    let now = std::time::SystemTime::now();
    let duration = now.elapsed().unwrap();
    z = (duration.as_secs() as u64) ^ (duration.subsec_micros() as u64) | 1;
    (z.wrapping_mul(3) >> 1) as usize % range
}

fn search(aa: &Vec<i32>, ii: &Vec<usize>, a: i32, i: usize) -> usize {
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