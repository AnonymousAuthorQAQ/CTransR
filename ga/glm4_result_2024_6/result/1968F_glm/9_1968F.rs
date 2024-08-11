use std::io;
use std::time::SystemTime;

const N: usize = 200000;

fn rand_() -> u32 {
    let z = unsafe { Z };
    Z = z.wrapping_mul(3).wrapping_shr(1);
    Z
}

static mut Z: u32 = 0;

fn srand_() {
    let now = SystemTime::now();
    let duration = now.elapsed().expect("Time went backwards");
    let micros = duration.subsec_micros() as u32;
    let secs = duration.as_secs() as u32;
    unsafe {
        Z = secs ^ micros | 1;
    }
}

fn sort(aa: &mut [i32], n: usize) {
    let mut ii = (0..=n).collect::<Vec<_>>();
    let mut l = 0;
    let mut r = n + 1;

    while l < r - 1 {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand_() as usize % (r - l - 1)];
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
        sort(&mut ii[l..i], i - l);
        l = k;
    }
}

fn search(a: i32, i: usize, aa: &[i32], ii: &[usize]) -> usize {
    let mut lower = -1;
    let mut upper = ii.len();

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if aa[ii[h]] < a || (aa[ii[h]] == a && ii[h] < i) {
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
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.trim().split_whitespace();
        let n: usize = iter.next().unwrap().parse().expect("Invalid input");
        let q: usize = iter.next().unwrap().parse().expect("Invalid input");

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            aa[i] = iter.next().unwrap().parse().expect("Invalid input");
            aa[i] ^= aa[i - 1];
        }

        let mut ii = (0..=n).collect::<Vec<_>>();
        sort(&mut ii, n);

        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let mut iter = input.trim().split_whitespace();
            let l: usize = iter.next().unwrap().parse().expect("Invalid input") - 1;
            let r: usize = iter.next().unwrap().parse().expect("Invalid input");

            println!(
                "{}",
                if aa[l] == aa[r] || ii[search(aa[r], l, &aa, &ii) + 1] < ii[search(aa[l], r, &aa, &ii)] {
                    "YES"
                } else {
                    "NO"
                }
            );
        }
    }
}