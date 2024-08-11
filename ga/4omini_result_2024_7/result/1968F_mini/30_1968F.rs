use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200_000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let z = (now.as_secs() ^ now.subsec_micros() as u64) as u32 | 1;
        Random { z }
    }

    fn rand(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
        self.z >> 1
    }
}

fn sort(ii: &mut Vec<usize>, aa: &[u32], l: usize, r: usize, rand: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + (rand.rand() % (r - l) as u32) as usize];
        let mut tmp;

        let mut j = j;
        let mut k = k;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] as i32 - aa[i_] as i32
            } else {
                ii[j] as i32 - i_ as i32
            };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                tmp = ii[i];
                ii[i] = ii[j];
                ii[j] = tmp;
                l += 1;
                j += 1;
            } else {
                k -= 1;
                tmp = ii[j];
                ii[j] = ii[k];
                ii[k] = tmp;
            }
        }
        sort(ii, aa, l, k, rand);
        l = k;
    }
}

fn search(aa: &[u32], ii: &[usize], a: u32, n: usize) -> isize {
    let mut lower = -1;
    let mut upper = n as isize + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] < n as usize) {
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

    for _ in 0..t {
        let first_line = lines.next().unwrap().unwrap();
        let mut parts = first_line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        let mut aa = vec![0u32; n + 1];
        let mut ii = (0..=n).collect::<Vec<usize>>();
        let mut rand = Random::new();

        for i in 1..=n {
            let num: u32 = lines.next().unwrap().unwrap().parse().unwrap();
            aa[i] = aa[i - 1] ^ num;
        }

        sort(&mut ii, &aa, 0, n + 1, &mut rand);

        for _ in 0..q {
            let query_line = lines.next().unwrap().unwrap();
            let mut query_parts = query_line.split_whitespace();
            let l: usize = query_parts.next().unwrap().parse().unwrap();
            let r: usize = query_parts.next().unwrap().parse().unwrap();
            let l = l - 1; // Convert to 0-based index

            let result = if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], n)] < ii[search(&aa, &ii, aa[l], n) + 1] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}