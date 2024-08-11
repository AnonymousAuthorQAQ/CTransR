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

fn sort(ii: &mut [usize], aa: &[u32], l: usize, r: usize, rng: &mut Random) {
    let mut l = l;
    let mut r = r;
    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = ii[l + (rng.rand() % (r - l) as u32) as usize];

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
                j += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        sort(ii, aa, l, i, rng);
        l = k;
    }
}

fn search(aa: &[u32], ii: &[usize], a: u32, n: usize) -> isize {
    let mut lower = -1;
    let mut upper = n as isize + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        if aa[ii[h as usize]] < a
            || (aa[ii[h as usize]] == a && ii[h as usize] < ii[lower as usize])
        {
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

    let mut rng = Random::new();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let input: u32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
            aa[i] = aa[i - 1] ^ input;
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, &aa, 0, n + 1, &mut rng);

        for _ in 0..q {
            let query_line = lines.next().unwrap().unwrap();
            let mut query_parts = query_line.split_whitespace();
            let mut l: usize = query_parts.next().unwrap().parse().unwrap();
            let r: usize = query_parts.next().unwrap().parse().unwrap();
            l -= 1; // Adjust for 0-based index

            let result = if aa[l] == aa[r]
                || ii[search(&aa, &ii, aa[r], n) as usize + 1] < ii[search(&aa, &ii, aa[l], n)]
            {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}