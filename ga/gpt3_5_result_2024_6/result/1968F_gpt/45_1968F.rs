use std::io::{self, BufRead};

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand_() -> i32 {
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        Z as i32
    }
}

fn srand_() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let tv = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    unsafe {
        Z = (tv.as_secs() ^ tv.as_micros() as u64) as u32 | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[u32]) {
    if l < r {
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let i_ = aa[ii[l + rand_() as usize % (r - l)]];
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != i_ {
                aa[ii[j]] - i_
            } else {
                ii[j] as u32 - i_
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
        sort(ii, l, i, aa);
        sort(ii, k, r, aa);
    }
}

fn search(a: u32, i: usize, ii: &[usize], aa: &[u32]) -> i32 {
    let mut lower = -1;
    let mut upper = ii.len() as i32 + 1;

    while upper - lower > 1 {
        let h = ((lower + upper) / 2) as usize;

        if aa[ii[h]] < a || (aa[ii[h]] == a && ii[h] < i) {
            lower = h as i32;
        } else {
            upper = h as i32;
        }
    }
    lower
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    srand_();

    let mut buffer = String::new();
    input.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let mut iter = buffer.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let aa: Vec<u32> = buffer.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, 0, n + 1, &aa);

        for _ in 0..q {
            buffer.clear();
            input.read_line(&mut buffer).unwrap();
            let mut iter = buffer.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap() - 1;
            let r: usize = iter.next().unwrap().parse().unwrap();

            let result = if aa[l] == aa[r] || ii[search(aa[r], l, &ii, &aa) as usize + 1] < ii[search(aa[l], r, &ii, &aa) as usize] {
                "YES"
            } else {
                "NO"
            };

            println!("{}", result);
        }
    }
}