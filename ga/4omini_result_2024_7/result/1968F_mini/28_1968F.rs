use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        // Get the current time in nanoseconds and use it to seed the random generator
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let seconds = since_epoch.as_secs() as u32;
        let nanoseconds = since_epoch.subsec_nanos() as u32;
        let seed = seconds ^ nanoseconds | 1;

        Random { z: seed }
    }

    fn rand(&mut self) -> u32 {
        self.z *= 3;
        self.z >> 1
    }
}

fn sort(aa: &[u32], ii: &mut Vec<usize>, l: usize, r: usize, rand: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let j = l;
        let k = r;

        let i_ = ii[l + (rand.rand() as usize % (r - l))];

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
        sort(aa, ii, l, k, rand);
        l = k;
    }
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
    let mut rand = Random::new();

    for _ in 0..t {
        let (n, q) = {
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        };

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let mut value: u32 = lines.next().unwrap().unwrap().parse().unwrap();
            aa[i] = aa[i - 1] ^ value;
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&aa, &mut ii, 0, n + 1, &mut rand);

        for _ in 0..q {
            let (l, r) = {
                let line = lines.next().unwrap().unwrap();
                let mut parts = line.split_whitespace();
                (
                    parts.next().unwrap().parse::<usize>().unwrap() - 1,
                    parts.next().unwrap().parse::<usize>().unwrap(),
                )
            };

            let response = if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l) as usize + 1] < ii[search(&aa, &ii, aa[l], r) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", response);
        }
    }
}