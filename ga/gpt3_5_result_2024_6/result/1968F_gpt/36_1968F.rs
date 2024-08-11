use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand_() -> i32 {
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        (Z as i32)
    }
}

fn srand_() {
    let tv = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    unsafe {
        Z = tv.as_secs() as u32 ^ tv.as_micros() as u32 | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[i32]) {
    if l < r {
        let i_ = l + rand_() as usize % (r - l);
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

fn search(a: i32, i: usize, ii: &[usize], aa: &[i32]) -> i32 {
    let mut lower = -1;
    let mut upper = ii.len() as i32 + 1;

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
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut nums = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());

        let n = nums.next().unwrap();
        let q = nums.next().unwrap();

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let aa: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        let mut ii: Vec<usize> = (0..=n as usize).collect();
        sort(&mut ii, 0, n as usize + 1, &aa);

        for _ in 0..q {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut queries = input.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap());

            let l = queries.next().unwrap() - 1;
            let r = queries.next().unwrap();

            let result = if aa[l] == aa[r] || ii[(search(aa[r], l, &ii, &aa) + 1) as usize] < ii[search(aa[l], r, &ii, &aa) as usize] {
                "YES"
            } else {
                "NO"
            };

            println!("{}", result);
        }
    }
}