use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

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

fn sort(ii: &mut [usize], aa: &[u32], l: usize, r: usize, rng: &mut Random) {
    if l >= r {
        return;
    }

    let mut i = l;
    let mut j = l;
    let mut k = r;
    let i_ = ii[l + (rng.rand() as usize % (r - l))];

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
    sort(ii, aa, l, i, rng);
    sort(ii, aa, k, r, rng);
}

fn search(aa: &[u32], ii: &[usize], a: u32, i: usize) -> isize {
    let mut lower = -1;
    let mut upper = ii.len() as isize;

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
    let mut rng = Random::new();

    for _ in 0..t {
        let (n, q): (usize, usize) = {
            let line = lines.next().unwrap().unwrap();
            let mut iter = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        };

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let x: u32 = lines.next().unwrap().unwrap().parse().unwrap();
            aa[i] = aa[i - 1] ^ x;
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, &aa, 0, n + 1, &mut rng);

        for _ in 0..q {
            let (l, r): (usize, usize) = {
                let line = lines.next().unwrap().unwrap();
                let mut iter = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
                (iter.next().unwrap() - 1, iter.next().unwrap())
            };

            let res = if aa[l] == aa[r] || ii[(search(&aa, &ii, aa[r], l) + 1) as usize] < ii[search(&aa, &ii, aa[l], r) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", res);
        }
    }
}