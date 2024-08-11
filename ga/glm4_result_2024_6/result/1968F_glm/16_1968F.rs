use std::io::{self, BufRead};
use std::time::SystemTime;

const N: usize = 200_000;

fn rand() -> u32 {
    static mut Z: u32 = 0;
    let now = SystemTime::now();
    let timestamp = now.elapsed().unwrap().as_nanos() as u32;
    unsafe {
        Z = Z.wrapping_add(timestamp);
        (Z.wrapping_mul(3)).rotate_right(1)
    }
}

fn srand() {
    let now = SystemTime::now();
    let timestamp = now.elapsed().unwrap().as_nanos() as u32;
    unsafe {
        Z = timestamp;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    if r - l > 1 {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand() as usize % (r - l)];
        while j < k {
            let c = if arr[ii[j]] != arr[i_] {
                arr[ii[j]] - arr[i_]
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
        sort(ii, l, i);
        sort(ii, k, r);
    }
}

fn search(a: u32, i: usize) -> isize {
    let mut lower = -1;
    let mut upper = arr.len() as isize + 1;
    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        if arr[ii[h as usize]] < a || (arr[ii[h as usize]] == a && ii[h as usize] < i) {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower
}

fn main() {
    srand();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n = parts.next().unwrap().parse::<usize>().unwrap();
        let q = parts.next().unwrap().parse::<usize>().unwrap();
        let mut arr = Vec::with_capacity(n + 1);
        arr.push(0);
        for _ in 1..=n {
            arr.push(arr.last().unwrap() ^ lines.next().unwrap().unwrap().parse::<u32>().unwrap());
        }
        let mut ii = (0..=n).collect::<Vec<_>>();
        sort(&mut ii, 0, n + 1);
        for _ in 0..q {
            let mut line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            let l = parts.next().unwrap().parse::<usize>().unwrap() - 1;
            let r = parts.next().unwrap().parse::<usize>().unwrap();
            println!("{}", if arr[l] == arr[r] || ii[search(arr[r], l) as usize + 1] < ii[search(arr[l], r)] {
                "YES"
            } else {
                "NO"
            });
        }
    }
}