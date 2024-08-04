use std::io::{self, BufRead};
use std::time::SystemTime;

const N: usize = 200_000;

static mut Z: u32 = 0;

fn rand_() -> u32 {
    unsafe {
        Z *= 3; // Adjust Z using the same multiplication
        Z >> 1
    }
}

fn srand_() {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards");
    let z_value = now.as_secs() ^ now.subsec_micros() | 1;
    unsafe {
        Z = z_value as u32; // Initialize Z with the derived value
    }
}

fn sort(ii: &mut [usize], aa: &[u32], l: usize, r: usize) {
    let mut l = l;
    let mut r = r;
    
    while l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l] + (rand_() % (r - l) as u32) as usize; // Use RDP for bounds
        let mut tmp;

        let mut j = j;
        let mut k = k;

        while j < k {
            let cmp_result = if aa[ii[j]] != aa[i_] {
                aa[ii[j]] as isize - aa[i_] as isize
            } else {
                ii[j] as isize - ii[i] as isize
            };

            if cmp_result == 0 {
                // Equal
                j += 1;
            } else if cmp_result < 0 {
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
        sort(ii, aa, l, i);
        l = k;
    }
}

fn search(aa: &[u32], ii: &[usize], a: u32, i: usize, n: usize) -> i32 {
    let mut lower = -1;
    let mut upper = n + 1;

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
    srand_(); // Seed random number generator

    let stdin = io::stdin();
    let reader = stdin.lock();
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut index = 0;
    let t: usize = lines[index].parse().unwrap();
    index += 1;

    for _ in 0..t {
        let line: Vec<usize> = lines[index].split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let n = line[0];
        let q = line[1];
        index += 1;
        
        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let value: u32 = lines[index].parse().unwrap();
            aa[i] = value ^ aa[i - 1];
            index += 1;
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, &aa, 0, n + 1);

        for _ in 0..q {
            let line: Vec<usize> = lines[index].split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let l = line[0] - 1;
            let r = line[1];

            let result = if aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l, n)] < ii[search(&aa, &ii, aa[l], r, n) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
            index += 1;
        }
    }
}