use std::io::{self, Read};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    pub fn new(seed: u32) -> Self {
        Self { z: seed }
    }

    pub fn rand(&mut self) -> u32 {
        self.z *= 3;
        self.z >> 1
    }
}

fn srand() -> u32 {
    let start = SystemTime::now();
    let duration = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    (duration.as_secs() ^ duration.subsec_micros()) | 1
}

fn sort(aa: &mut Vec<u32>, ii: &mut Vec<usize>, l: usize, r: usize) {
    if l < r {
        let mut i = l;
        let mut j = l;
        let mut k = r;

        let rand = ii[l] + rand() % (r - l);
        let pivot = aa[ii[rand]];

        while j < k {
            let c = if aa[ii[j]] != pivot {
                aa[ii[j]] as i32 - pivot as i32
            } else {
                ii[j] as i32 - ii[i] as i32
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
        sort(aa, ii, l, i);
        sort(aa, ii, k, r);
    }
}

fn search(aa: &Vec<u32>, ii: &Vec<usize>, a: u32, i: usize) -> usize {
    let mut lower = !0;
    let mut upper = ii.len();

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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    let mut srand_value = srand();
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let (n, q) = {
            let para = lines.next().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            (para[0], para[1])
        };

        let mut aa = vec![0; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();
        
        for i in 1..=n {
            let num: u32 = lines.next().unwrap().parse().unwrap();
            aa[i] = num ^ aa[i - 1];
        }

        sort(&mut aa, &mut ii, 0, n + 1);

        for _ in 0..q {
            let (l, r) = {
                let para = lines.next().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                (para[0] - 1, para[1])  // Adjust l (0-based index)
            };

            let contains = aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l) + 1] < ii[search(&aa, &ii, aa[l], r)];

            if contains {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}