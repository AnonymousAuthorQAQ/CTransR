use std::io::{self, BufRead};
use std::time::SystemTime;

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
        let seconds = since_epoch.as_secs() as u32;
        let micros = since_epoch.subsec_micros() as u32;
        let z = seconds ^ micros | 1;
        Random { z }
    }

    fn rand(&mut self) -> u32 {
        self.z *= 3;
        self.z >> 1
    }
}

fn sort(ii: &mut [usize], aa: &[u32], l: usize, r: usize, rand: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + (rand.rand() % (r - l) as usize)];

        let mut j = j;
        let mut k = k;
        let mut i = i;

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

fn search(a: u32, i: usize, aa: &[u32], ii: &[usize]) -> usize {
    let mut lower = -1_isize;
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
    let reader = stdin.lock();
    let mut lines = reader.lines().map(|line| line.unwrap());

    let mut rand = Random::new();
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let (n, q): (usize, usize) = {
            let nums: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
            (nums[0], nums[1])
        };

        let mut aa = vec![0u32; n + 1];
        for i in 1..=n {
            let value: u32 = lines.next().unwrap().parse().unwrap();
            aa[i] = value ^ aa[i - 1];
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, &aa, 0, n + 1, &mut rand);

        for _ in 0..q {
            let (l, r): (usize, usize) = {
                let nums: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
                (nums[0] - 1, nums[1]) // l is 0-indexed, r is inclusive
            };

            let condition = aa[l] == aa[r] || ii[search(aa[r], l, &aa, &ii) + 1] < ii[search(aa[l], r, &aa, &ii)];
            println!("{}", if condition { "YES" } else { "NO" });
        }
    }
}