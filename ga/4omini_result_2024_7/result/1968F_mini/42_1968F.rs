use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200_000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let seed = (since_epoch.as_secs() ^ since_epoch.subsec_micros() as u64) as u32 | 1;
        Random { z: seed }
    }

    fn rand(&mut self) -> u32 {
        self.z *= 3;
        self.z >> 1
    }
}

fn sort(aa: &mut [u32], ii: &mut [usize], l: usize, r: usize, random: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let mut k = k;
        let i_ = ii[l + (random.rand() as usize % (r - l))];

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] as i32 - aa[i_] as i32
            } else {
                ii[j] as i32 - i_ as i32
            };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                ii.swap(i, j);
                l += 1;
                j += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        sort(aa, ii, l, i, random);
        l = k;
    }
}

fn search(aa: &[u32], ii: &[usize], n: usize, a: u32, i: usize) -> isize {
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
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut random = Random::new();

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = parts[0];
        let q = parts[1];

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let mut num: u32 = lines.next().unwrap().unwrap().parse().unwrap();
            aa[i] = num ^ aa[i - 1];
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut aa, &mut ii, 0, n + 1, &mut random);

        for _ in 0..q {
            let line = lines.next().unwrap().unwrap();
            let parts: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            let l = parts[0] - 1;
            let r = parts[1];

            if aa[l] == aa[r] || ii[search(&aa, &ii, n, aa[r], l) as usize + 1] < ii[search(&aa, &ii, n, aa[l], r) as usize] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}