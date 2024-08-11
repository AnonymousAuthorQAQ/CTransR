use std::io::{self, Read};
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200_000;

struct Random {
    z: u32,
}

impl Random {
    fn new() -> Self {
        // Seed the RNG with current timestamp.
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let seed = (now.as_secs() ^ now.subsec_millis() as u64) as u32 | 1;
        Random { z: seed }
    }

    fn rand(&mut self) -> u32 {
        self.z *= 3;
        self.z >> 1
    }
}

fn sort(ii: &mut Vec<usize>, aa: &Vec<u32>, l: usize, r: usize, rng: &mut Random) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let rand_index = l + (rng.rand() % (r - l) as u32) as usize;
        let i_ = aa[ii[rand_index]];

        let mut j = j;

        while j < k {
            let c = if aa[ii[j]] != i_ {
                aa[ii[j]] as i32 - i_ as i32
            } else {
                ii[j] as i32 - ii[i] as i32
            };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                ii.swap(i, j);
                j += 1;
                l += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        sort(ii, aa, i, l, rng);
        l = k;
    }
}

fn search(aa: &Vec<u32>, ii: &Vec<usize>, a: u32, n: usize) -> isize {
    let mut lower = -1;
    let mut upper = n as isize + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] < 0) {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().bytes()
        .filter_map(Result::ok)
        .collect::<Vec<u8>>();

    let mut idx = 0;

    fn read_next(input: &[u8], idx: &mut usize) -> (usize, usize) {
        let mut value = 0;
        while *idx < input.len() && input[*idx] != b' ' && input[*idx] != b'\n' {
            value = value * 10 + (input[*idx] - b'0') as usize;
            *idx += 1;
        }
        while *idx < input.len() && (input[*idx] == b' ' || input[*idx] == b'\n') {
            *idx += 1;
        }
        value
    }

    let t = read_next(&input, &mut idx);
    for _ in 0..t {
        let n = read_next(&input, &mut idx);
        let q = read_next(&input, &mut idx);

        let mut aa = vec![0u32; n + 1];
        for i in 1..=n {
            let mut value = read_next(&input, &mut idx);
            aa[i] = value as u32 ^ aa[i - 1];
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        let mut rng = Random::new();
        sort(&mut ii, &aa, 0, n + 1, &mut rng);

        for _ in 0..q {
            let l = read_next(&input, &mut idx) - 1; // l is 0-based after decrement
            let r = read_next(&input, &mut idx);

            let answer = if aa[l] == aa[r as usize] || ii[search(&aa, &ii, aa[r as usize], n)] < ii[search(&aa, &ii, aa[l], n) + 1] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", answer);
        }
    }
}