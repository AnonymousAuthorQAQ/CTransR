use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{self, Read};
use std::cmp::Ordering;

const N: usize = 200000;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let lines: Vec<&str> = buffer.lines().collect();

    let t: usize = lines[0].parse().unwrap();
    let mut index = 1;

    for _ in 0..t {
        let n: usize = lines[index].parse().unwrap();
        let q: usize = lines[index + 1].parse().unwrap();
        index += 2;

        let mut aa: Vec<i32> = lines[index..index + n].iter().map(|s| s.parse().unwrap()).collect();
        index += n;

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut aa, &mut ii);

        for _ in 0..q {
            let l: usize = lines[index].parse().unwrap() - 1;
            let r: usize = lines[index + 1].parse().unwrap();
            index += 2;

            println!("{}", if aa[l] == aa[r] || ii[search(&aa, &ii, l)] < ii[search(&aa, &ii, r)] {
                "YES"
            } else {
                "NO"
            });
        }
    }
}

fn sort(aa: &mut Vec<i32>, ii: &mut Vec<usize>) {
    if ii.len() <= 1 {
        return;
    }

    let mut rng = rand::thread_rng();
    let mut i = 0;
    while i < ii.len() {
        let mut j = i;
        while j < ii.len() {
            if aa[ii[j]] < aa[ii[i]] || (aa[ii[j]] == aa[ii[i]] && ii[j] < ii[i]) {
                ii.swap(i, j);
            }
            j += 1;
        }
        i += 1;
    }
}

fn search(aa: &Vec<i32>, ii: &Vec<usize>, i: usize) -> usize {
    let mut lower = -1;
    let mut upper = ii.len();

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        match aa[ii[h]].cmp(&aa[i]) {
            Ordering::Less => lower = h,
            Ordering::Equal => {
                if ii[h] < i {
                    lower = h;
                } else {
                    upper = h;
                }
            }
            Ordering::Greater => upper = h,
        }
    }

    lower
}

fn rand_() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn srand_() {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    rand::SeedableRng::seed_from_u64(since_the_epoch.as_secs() as u64 ^ since_the_epoch.subsec_nanos() as u64);
}