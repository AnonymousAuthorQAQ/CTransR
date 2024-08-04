use std::io::{self, BufRead};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200_000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let z = (start.as_secs() ^ start.subsec_millis()) | 1;
        Random { z: z as u32 }
    }
    
    fn rand(&mut self) -> u32 {
        self.z = self.z.wrapping_mul(3);
        self.z >> 1
    }
}

fn sort(ii: &mut [usize], aa: &[u32], l: usize, r: usize, random: &mut Random) {
    if l >= r {
        return;
    }

    let mut i = l;
    let mut j = l;
    let mut k = r;
    let i_ = ii[l + (random.rand() % (r - l) as usize)];

    while j < k {
        let c = match aa[ii[j]].cmp(&aa[i_]) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => ii[j].cmp(&ii[i]),
        };

        match c {
            std::cmp::Ordering::Equal => j += 1,
            std::cmp::Ordering::Less => {
                ii.swap(i, j);
                i += 1;
                j += 1;
            }
            std::cmp::Ordering::Greater => {
                k -= 1;
                ii.swap(j, k);
            }
        }
    }
    sort(ii, aa, l, i, random);
    sort(ii, aa, k, r, random);
}

fn search(ii: &[usize], aa: &[u32], a: u32, n: usize, i: usize) -> usize {
    let mut lower = usize::MAX;
    let mut upper = n + 1;

    while upper > lower + 1 {
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
    let reader = stdin.lock();
    let mut lines = reader.lines().map(|line| line.unwrap());
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let mut input = lines.next().unwrap().split_whitespace();
        let n: usize = input.next().unwrap().parse().unwrap();
        let q: usize = input.next().unwrap().parse().unwrap();

        let mut aa = vec![0u32; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();
        let mut random = Random::new();

        for i in 1..=n {
            let a: u32 = lines.next().unwrap().parse().unwrap();
            aa[i] = aa[i - 1] ^ a;
        }

        sort(&mut ii, &aa, 0, n + 1, &mut random);

        for _ in 0..q {
            let mut input = lines.next().unwrap().split_whitespace();
            let l: usize = input.next().unwrap().parse().unwrap();
            let r: usize = input.next().unwrap().parse().unwrap();
            let result = if aa[l] == aa[r] || ii[search(&ii, &aa, aa[r], n, l) + 1] < ii[search(&ii, &aa, aa[l], n, r)] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}