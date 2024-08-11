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
    let tv = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as u32;
    unsafe {
        Z = tv ^ 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &Vec<i32>) {
    let mut l = l;
    let mut r = r;
    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let mut i_ = aa[ii[l + (rand_() as usize) % (r - l)];
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != i_ {
                aa[ii[j]] - i_
            } else {
                ii[j] - i_
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
        l = k;
    }
}

fn search(a: i32, i: usize, ii: &Vec<usize>, aa: &Vec<i32>) -> isize {
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
    unsafe {
        srand_();
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let aa: Vec<i32> = input.split_whitespace()
                                .map(|x| x.parse().unwrap())
                                .scan(0, |state, x| {
                                    *state ^= x;
                                    Some(*state)
                                })
                                .collect();

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, 0, n + 1, &aa);

        for _ in 0..q {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();

            let l = l - 1;
            let idx1 = search(aa[r], l, &ii, &aa) as usize;
            let idx2 = search(aa[l], r, &ii, &aa) as usize;

            if aa[l] == aa[r] || ii[idx1 + 1] < ii[idx2] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}