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
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    unsafe {
        Z = (since_the_epoch.as_secs() as u32) ^ (since_the_epoch.as_micros() as u32) | 1;
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
            let c = if aa[ii[j]] != i_ { aa[ii[j]] - i_ } else { ii[j] - i_ };

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

fn search(a: u32, i: usize, aa: &[u32], ii: &[usize]) -> isize {
    let mut lower = -1;
    let mut upper = (aa.len() + 1) as isize;

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
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: i32 = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().expect("Missing value").parse().expect("Invalid input");
        let q: usize = iter.next().expect("Missing value").parse().expect("Invalid input");

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            aa[i] = input.trim().parse().expect("Invalid input");
            aa[i] ^= aa[i - 1];
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, 0, n + 1, &aa);

        for _ in 0..q {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            let mut iter = input.split_whitespace();
            let l: usize = iter.next().expect("Missing value").parse().expect("Invalid input");
            let r: usize = iter.next().expect("Missing value").parse().expect("Invalid input");

            if aa[l] == aa[r] || ii[(search(aa[r], l, &aa, &ii) + 1) as usize] < ii[search(aa[l], r, &aa, &ii) as usize] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}