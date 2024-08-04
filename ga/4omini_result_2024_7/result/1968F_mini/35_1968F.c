use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{self, Read};

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let z = (now.as_secs() ^ now.subsec_nanos() as u64) as u32 | 1;
        Random { z }
    }

    fn rand(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
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
        let i_ = ii[l] + (random.rand() % (r - l) as u32) as usize;

        let mut j = j;
        let mut k = k;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] as i32 - aa[i_] as i32
            } else {
                ii[j] as i32 - ii[i] as i32
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
        sort(aa, ii, l, k, random);
        l = k;
    }
}

fn search(aa: &[u32], ii: &[usize], a: u32, n: usize, i: usize) -> usize {
    let mut lower = !0;
    let mut upper = n + 1;

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
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    
    let mut lines = input.lines();
    let mut rng = Random::new();

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let (n, q): (usize, usize) = {
            let line = lines.next().unwrap();
            let mut split = line.split_whitespace();
            (split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap())
        };

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let x: u32 = lines.next().unwrap().parse().unwrap();
            aa[i] = x ^ aa[i - 1];
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut aa, &mut ii, 0, n + 1, &mut rng);

        for _ in 0..q {
            let (l, r): (usize, usize) = {
                let line = lines.next().unwrap();
                let mut split = line.split_whitespace();
                (split.next().unwrap().parse::<usize>().unwrap() - 1, split.next().unwrap().parse::<usize>().unwrap())
            };

            let res = if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], n, l) + 1] < ii[search(&aa, &ii, aa[l], n, r)] {
                "YES"
            } else {
                "NO"
            };

            println!("{}", res);
        }
    }
}