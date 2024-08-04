use std::io::{self, BufRead};
use std::time::SystemTime;

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let now = SystemTime::now();
        let seconds = now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let microseconds = now.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_micros();
        let z = (seconds ^ microseconds) | 1;
        Random { z }
    }

    fn rand(&mut self) -> u32 {
        self.z *= 3;
        self.z >> 1
    }
}

fn sort(ii: &mut Vec<usize>, aa: &[u32], l: usize, r: usize, rand: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = ii[l] + (rand.rand() % ((r - l) as u32)) as usize;
        let tmp_i_val = aa[ii[l]];

        while j < k {
            let c = if aa[ii[j]] != tmp_i_val {
                aa[ii[j]] - tmp_i_val
            } else {
                ii[j] - ii[i]
            };

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
        sort(ii, aa, l, i, rand);
        l = k;
    }
}

fn search(aa: &[u32], ii: &[usize], a: u32, i: usize, n: usize) -> isize {
    let mut lower = -1;
    let mut upper = n as isize + 1;

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
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().flat_map(Result::ok);

    let mut rand = Random::new();
    let t: usize = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let (n, q): (usize, usize) = {
            let line = lines.next().unwrap();
            let mut split = line.split_whitespace();
            let n = split.next().unwrap().parse().unwrap();
            let q = split.next().unwrap().parse().unwrap();
            (n, q)
        };

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let num: u32 = lines.next().unwrap().trim().parse().unwrap();
            aa[i] = aa[i - 1] ^ num;
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, &aa, 0, n + 1, &mut rand);

        for _ in 0..q {
            let (l, r): (usize, usize) = {
                let line = lines.next().unwrap();
                let mut split = line.split_whitespace();
                let l = split.next().unwrap().parse().unwrap();
                let r = split.next().unwrap().parse().unwrap();
                (l - 1, r) // Convert to zero-indexed
            };

            let condition = aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l, n)] < ii[search(&aa, &ii, aa[l], r, n)];
            if condition {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}