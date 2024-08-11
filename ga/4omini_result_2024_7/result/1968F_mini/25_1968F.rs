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
        self.z *= 3;
        self.z >> 1
    }
}

fn sort(ii: &mut Vec<usize>, aa: &Vec<u32>, l: usize, r: usize, rand: &mut Random) {
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

fn search(a: u32, i: usize, aa: &Vec<u32>, ii: &Vec<usize>, n: usize) -> usize {
    let mut lower = -1i32;
    let mut upper = n as i32 + 1;

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

    let t: usize = lines.next().unwrap().parse().unwrap();
    let mut rand = Random::new();

    for _ in 0..t {
        let (n, q): (usize, usize) = {
            let parts: Vec<usize> = lines.next().unwrap().split_whitespace()
                .map(|x| x.parse().unwrap()).collect();
            (parts[0], parts[1])
        };

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let value: u32 = lines.next().unwrap().parse().unwrap();
            aa[i] = aa[i - 1] ^ value;
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, &aa, 0, n + 1, &mut rand);

        for _ in 0..q {
            let (l, r): (usize, usize) = {
                let parts: Vec<usize> = lines.next().unwrap().split_whitespace()
                    .map(|x| x.parse().unwrap()).collect();
                (parts[0] - 1, parts[1])
            };

            let result = if aa[l] == aa[r] || ii[search(aa[r], l, &aa, &ii, n)] < ii[search(aa[l], r, &aa, &ii, n)] {
                "YES"
            } else {
                "NO"
            };

            println!("{}", result);
        }
    }
}