use std::io;
use std::time::SystemTime;

const N: usize = 200000;

fn rand() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = Z.wrapping_mul(3).wrapping_shr(1);
        Z
    }
}

fn srand() {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards");
    let seed = since_epoch.as_secs() as u32 ^ since_epoch.subsec_nanos() as u32 | 1;
    unsafe {
        Z = seed;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    if l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand() as usize % (r - l)];
        let mut tmp;

        while j < k {
            let c = if ii[j] != i_ { ii[j] - i_ } else { ii[j] - i_ };

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
        sort(ii, l, i);
        sort(ii, k, r);
    }
}

fn search(a: usize, i: usize, aa: &[usize], ii: &[usize]) -> usize {
    let mut lower = -1;
    let mut upper = ii.len() + 1;

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
    srand();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().expect("Invalid input").parse().expect("Invalid input");
        let q: usize = parts.next().expect("Invalid input").parse().expect("Invalid input");

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            aa[i] = input.trim().parse().expect("Invalid input") ^ aa[i - 1];
        }

        let mut ii = (0..=n).collect::<Vec<usize>>();
        sort(&mut ii, 0, n + 1);

        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let mut parts = input.trim().split_whitespace();
            let l: usize = parts.next().expect("Invalid input").parse().expect("Invalid input") - 1;
            let r: usize = parts.next().expect("Invalid input").parse().expect("Invalid input");

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