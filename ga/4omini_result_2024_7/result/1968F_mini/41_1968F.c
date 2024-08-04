use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200_000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let seed = (now.as_secs() ^ now.subsec_millis() as u64) as u32 | 1;
        Random { z: seed }
    }

    fn rand(&mut self) -> u32 {
        self.z *= 3;
        self.z >> 1
    }
}

fn sort(ii: &mut Vec<usize>, aa: &[u32], l: usize, r: usize, rand: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = ii[l + (rand.rand() as usize) % (r - l)];
        let mut tmp;

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
                i += 1;
                j += 1;
            } else {
                k -= 1;
                tmp = ii[j];
                ii[j] = ii[k];
                ii[k] = tmp;
            }
        }
        sort(ii, aa, l, i, rand);
        l = k;
    }
}

fn search(aa: &[u32], ii: &[usize], a: u32, i: usize) -> isize {
    let mut lower = -1;
    let mut upper = ii.len() as isize - 1;

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
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut params = input.split_whitespace();
        let n: usize = params.next().unwrap().parse().unwrap();
        let q: usize = params.next().unwrap().parse().unwrap();

        let mut aa = vec![0u32; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();
        let mut rand = Random::new();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let values: Vec<u32> = input.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        for i in 1..=n {
            aa[i] = aa[i - 1] ^ values[i - 1];
        }

        sort(&mut ii, &aa, 0, n + 1, &mut rand);

        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let mut bounds = input.split_whitespace();
            let l: usize = bounds.next().unwrap().parse::<usize>().unwrap() - 1;
            let r: usize = bounds.next().unwrap().parse::<usize>().unwrap();

            if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l) as usize + 1] < ii[search(&aa, &ii, aa[l], r) as usize] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}