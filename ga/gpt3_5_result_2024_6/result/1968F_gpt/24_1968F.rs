use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand_() -> i32 {
    let z_val: u32;
    unsafe {
        Z = Z.wrapping_mul(3);
        z_val = Z >> 1;
    }
    z_val as i32
}

fn srand_() {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    unsafe {
        Z = (since_the_epoch.as_secs() as u32) ^ (since_the_epoch.as_micros() as u32) | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[u32]) {
    if l < r {
        let i_ = ii[l + (rand_() as usize) % (r - l)];
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] - aa[i_]
            } else {
                ii[j] - i_
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
        sort(ii, l, i, aa);
        sort(ii, k, r, aa);
    }
}

fn search(a: u32, i: usize, aa: &[u32], ii: &[usize]) -> usize {
    let mut lower = -1;
    let mut upper = aa.len() as isize + 1;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] < i) {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower as usize
}

fn main() {
    srand_();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let mut aa: Vec<u32> = Vec::with_capacity(n + 1);
        aa.push(0); // dummy element to start from index 1
        for _ in 0..n {
            aa.push(iter.next().unwrap().parse().unwrap() ^ aa.last().unwrap());
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, 0, n + 1, &aa);

        for _ in 0..q {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap() - 1;
            let r: usize = iter.next().unwrap().parse().unwrap();

            let search_val1 = search(aa[r], l, &aa, &ii) + 1;
            let search_val2 = search(aa[l], r, &aa, &ii);

            if aa[l] == aa[r] || search_val1 < search_val2 {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}