use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200_000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let z = (start.as_secs() ^ start.subsec_micros() as u64) as u32 | 1;
        Random { z }
    }

    fn rand(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
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
        let mut tmp;

        let mut j = j;
        let mut k = k;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]].cmp(&aa[i_])
            } else {
                ii[j].cmp(&ii[i])
            };

            match c {
                std::cmp::Ordering::Equal => j += 1,
                std::cmp::Ordering::Less => {
                    tmp = ii[i];
                    ii[i] = ii[j];
                    ii[j] = tmp;
                    j += 1;
                    i += 1;
                }
                std::cmp::Ordering::Greater => {
                    k -= 1;
                    tmp = ii[j];
                    ii[j] = ii[k];
                    ii[k] = tmp;
                }
            }
        }

        sort(ii, aa, l, i, rand);
        l = k;
    }
}

fn search(a: u32, i: usize, ii: &[usize], aa: &[u32], n: usize) -> isize {
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
    let mut lines = reader.lines().map(|line| line.unwrap());
    
    let mut rand = Random::new();

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        let mut aa = vec![0u32; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();

        // Read array elements
        for i in 1..=n {
            let num: u32 = lines.next().unwrap().parse().unwrap();
            aa[i] = num ^ aa[i - 1];
        }

        sort(&mut ii, &aa, 0, n + 1, &mut rand);

        // Processing queries
        for _ in 0..q {
            let line = lines.next().unwrap();
            let mut iter = line.split_whitespace();
            let l: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
            let r: usize = iter.next().unwrap().parse::<usize>().unwrap();

            let result = if aa[l] == aa[r]
                || ii[search(aa[r], l, &ii, &aa, n)] < ii[search(aa[l], r, &ii, &aa, n)] {
                "YES"
            } else {
                "NO"
            };

            println!("{}", result);
        }
    }
}