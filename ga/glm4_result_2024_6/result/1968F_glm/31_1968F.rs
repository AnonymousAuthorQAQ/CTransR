use std::io;
use std::time::SystemTime;

const N: usize = 200000;

fn rand_() -> u32 {
    unsafe {
        let mut z = Z.lock().unwrap();
        *z = z.wrapping_mul(3);
        *z = z.rotate_right(1);
        *z
    }
}

fn srand_() {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let micros = since_epoch.subsec_micros();
    let secs = since_epoch.as_secs();
    unsafe {
        *Z.lock().unwrap() = (secs as u32) ^ micros as u32 | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    if l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand_() as usize % (r - l)];
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
        sort(ii, l, i);
        sort(ii, k, r);
    }
}

fn search(a: u32, i: usize) -> isize {
    let mut lower = -1;
    let mut upper = N as isize + 1;
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
    let t = read_int();

    for _ in 0..t {
        let (n, q) = (read_int(), read_int());
        let mut aa = [0; N + 1];
        for i in 1..=n {
            aa[i] = read_int();
            aa[i] ^= aa[i - 1];
        }
        let mut ii = (0..=n).collect::<Vec<usize>>();
        sort(&mut ii, 0, n + 1);
        for _ in 0..q {
            let (l, r) = (read_int() - 1, read_int());
            println!("{}", if aa[l] == aa[r] || ii[search(aa[r], l) as usize + 1] < ii[search(aa[l], r) as usize] { "YES" } else { "NO" });
        }
    }
}

fn read_int() -> u32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

use std::sync::{Mutex, Once};
use std::ops::ShlAssign;

static Z: Mutex<u32> = Mutex::new(0);
static ONCE: Once = Once::new();

fn main() {
    ONCE.call_once(|| {
        srand_();
    });
    // Rest of the code...
}