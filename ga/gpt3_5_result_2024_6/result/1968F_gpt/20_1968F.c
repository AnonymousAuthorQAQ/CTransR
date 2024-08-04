use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand_() -> i32 {
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        return Z as i32;
    }
}

fn srand_() {
    let tv = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();

    unsafe {
        Z = (tv.as_secs() ^ tv.subsec_micros() as u64) as u32 | 1;
    }
}

fn sort(aa: &mut [i32], ii: &mut [usize], l: usize, r: usize) {
    if l < r {
        let i_ = ii[l + rand_() as usize % (r - l)];
        let mut i = l;
        let mut j = l;
        let mut k = r;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] - aa[i_]
            } else {
                ii[j] as i32 - i_ as i32
            };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                ii.swap(i, j);
                i += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        sort(aa, ii, l, i);
        sort(aa, ii, k, r);
    }
}

fn search(aa: &mut [i32], ii: &mut [usize], a: i32, i: usize) -> i32 {
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
    return lower;
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut nums = input.trim().split_whitespace();

        let n: usize = nums.next().unwrap().parse().unwrap();
        let q: usize = nums.next().unwrap().parse().unwrap();

        let mut aa = vec![0; n + 1];
        let mut ii = vec![0; n + 1];

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut nums = input.trim().split_whitespace();

        for i in 1..=n {
            let num: i32 = nums.next().unwrap().parse().unwrap();
            aa[i] = aa[i - 1] ^ num;
        }

        for i in 0..=n {
            ii[i] = i;
        }

        sort(&mut aa, &mut ii, 0, n + 1);

        for _ in 0..q {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut nums = input.trim().split_whitespace();

            let l: usize = nums.next().unwrap().parse().unwrap();
            let r: usize = nums.next().unwrap().parse().unwrap();

            if aa[l] == aa[r] || ii[(search(&mut aa, &mut ii, aa[r], l) + 1) as usize] < ii[search(&mut aa, &mut ii, aa[l], r) as usize] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}