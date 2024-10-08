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
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        Z = (time.as_secs() as u32) ^ time.subsec_micros();
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[i32]) {
    if l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = aa[ii[l + rand_() as usize % (r - l)]];
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

fn search(a: i32, i: usize, aa: &[i32], ii: &[usize]) -> isize {
    let mut lower = -1;
    let mut upper = (ii.len() + 1) as isize;

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
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    srand_();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let aa: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let mut aa_xor: Vec<i32> = Vec::with_capacity(n + 1);
        aa_xor.push(0);
        for i in 1..=n {
            aa_xor.push(aa_xor[i - 1] ^ aa[i - 1]);
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, 0, n + 1, &aa_xor);

        for _ in 0..q {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();

            let result = if aa_xor[l] == aa_xor[r] || ii[search(aa_xor[r], l, &aa_xor, &ii) as usize + 1] < ii[search(aa_xor[l], r, &aa_xor, &ii) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}