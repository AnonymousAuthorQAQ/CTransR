use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Random {
        let z = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32 ^ SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_micros();
        Random { z: z }
    }

    fn rand(&mut self) -> i32 {
        self.z = self.z.wrapping_mul(3) >> 1;
        self.z as i32
    }
}

fn sort(aa: &mut [i32], ii: &mut [usize], l: usize, r: usize, random: &mut Random) {
    if l < r {
        let i_ = ii[l + (random.rand() as usize) % (r - l)];
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] { aa[ii[j]] - aa[i_] } else { ii[j] as i32 - i_ as i32 };

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
        sort(aa, ii, l, i, random);
        sort(aa, ii, k, r, random);
    }
}

fn search(aa: &[i32], ii: &[usize], a: i32, i: usize) -> i32 {
    let mut lower = -1;
    let mut upper = (ii.len() + 1) as i32;

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
    let mut random = Random::new();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();
        let mut aa: Vec<i32> = Vec::with_capacity(n + 1);
        aa.push(0);

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        for num in input.split_whitespace() {
            let num: i32 = num.parse().unwrap();
            aa.push(aa.last().unwrap() ^ num);
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut aa, &mut ii, 0, n + 1, &mut random);

        for _ in 0..q {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap() - 1;
            let r: usize = iter.next().unwrap().parse().unwrap();

            let result = if aa[l] == aa[r] || ii[(search(&aa, &ii, aa[r], l) + 1) as usize] < ii[search(&aa, &ii, aa[l], r) as usize] {
                "YES"
            } else {
                "NO"
            };

            println!("{}", result);
        }
    }
}