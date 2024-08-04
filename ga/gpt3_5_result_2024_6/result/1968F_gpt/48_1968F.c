use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

static mut Z: u32 = 0;

unsafe fn rand_() -> i32 {
    let z = &mut Z as *mut u32;
    *z = Z.wrapping_mul(3) >> 1;
    *z as i32
}

unsafe fn srand_() {
    let mut tv = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros() as u64;
    tv = tv | 1;
    Z = tv as u32;
}

fn sort(ii: &mut [i32], l: usize, r: usize, aa: &Vec<i32>) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let i_ = aa[ii[l + (rand_() as usize) % (r - l)];

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
        l = k;
    }
}

fn search(a: i32, i: i32, aa: &Vec<i32>, ii: &Vec<i32>) -> i32 {
    let mut lower = -1;
    let mut upper = aa.len() as i32 + 1;

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

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let t: i32 = input.trim().parse().unwrap();

        for _ in 0..t {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut numbers = input.trim().split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let n = numbers[0] as usize;
            let q = numbers[1];

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            numbers = input.trim().split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            let mut aa = vec![0; n + 1];
            aa[0] = 0;
            for i in 1..=n {
                aa[i] = aa[i - 1] ^ numbers[i - 1];
            }

            let mut ii: Vec<i32> = (0..=n as i32).collect();
            sort(&mut ii, 0, n + 1, &aa);

            for _ in 0..q {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                numbers = input.trim().split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();

                let l = numbers[0] - 1;
                let r = numbers[1];

                if aa[l as usize] == aa[r as usize] || ii[(search(aa[r], l) + 1) as usize] < ii[search(aa[l], r) as usize] {
                    println!("YES");
                } else {
                    println!("NO");
                }
            }
        }
    }
}