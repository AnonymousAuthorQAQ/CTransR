use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand_() -> i32 {
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        Z as i32
    }
}

fn srand_() {
    unsafe {
        let tv = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        Z = (tv.as_secs() ^ tv.subsec_micros()) as u32 | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[u32]) {
    if l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = aa[ii[l + (rand_() as usize) % (r - l)];
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] { aa[ii[j]] - aa[i_] } else { ii[j] - i_ };

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

fn search(a: u32, i: usize, ii: &[usize], aa: &[u32]) -> isize {
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
    srand_();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            aa[i] = input.trim().parse().unwrap();
            aa[i] ^= aa[i - 1];
        }

        let mut ii = (0..=n).collect::<Vec<usize>>();
        sort(&mut ii, 0, n + 1, &aa);

        for _ in 0..q {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();
            let l = l - 1;

            let result = if aa[l] == aa[r] || ii[search(aa[r], l, &ii, &aa) as usize + 1] < ii[search(aa[l], r, &ii, &aa) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}