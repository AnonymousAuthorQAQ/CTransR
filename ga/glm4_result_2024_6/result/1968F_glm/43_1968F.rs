use std::io::{self, BufRead};
use std::time::SystemTime;

const N: usize = 200_000;

fn main() {
    let stdin = io::stdin();
    let mut input_lines = stdin.lock().lines();

    let t: usize = input_lines.next().unwrap().expect("Invalid input").parse().expect("Invalid number");

    for _ in 0..t {
        let line1 = input_lines.next().unwrap().expect("Invalid input");
        let mut parts = line1.split_whitespace();
        let n: usize = parts.next().unwrap().parse().expect("Invalid number");
        let q: usize = parts.next().unwrap().parse().expect("Invalid number");

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let a: i32 = parts.next().unwrap().parse().expect("Invalid number");
            aa[i] = a ^ aa[i - 1];
        }

        let mut ii = (0..=n).collect::<Vec<usize>>();

        quicksort(&mut ii, &aa, 0, n);

        for _ in 0..q {
            let line2 = input_lines.next().unwrap().expect("Invalid input");
            let mut parts = line2.split_whitespace();
            let l: usize = parts.next().unwrap().parse().expect("Invalid number");
            let r: usize = parts.next().unwrap().parse().expect("Invalid number");

            println!("{}", if aa[l - 1] == aa[r - 1] || (l < n && aa[l] != aa[l + 1] && r < n && aa[r] != aa[r + 1]) {
                "YES"
            } else {
                "NO"
            });
        }
    }
}

fn quicksort(ii: &mut [usize], aa: &[i32], l: usize, r: usize) {
    if l < r {
        let pivot = partition(ii, aa, l, r);
        quicksort(ii, aa, l, pivot);
        quicksort(ii, aa, pivot + 1, r);
    }
}

fn partition(ii: &mut [usize], aa: &[i32], l: usize, r: usize) -> usize {
    let mut rng = rand::thread_rng();
    let pivot_index = l + rng.gen_range(l, r);
    ii.swap(l, pivot_index);

    let pivot = aa[ii[l]];
    let mut i = l;

    for j in l + 1..r {
        if aa[ii[j]] < pivot || (aa[ii[j]] == pivot && ii[j] < ii[l]) {
            i += 1;
            ii.swap(i, j);
        }
    }

    ii.swap(l, i);
    i
}

fn random() -> i32 {
    let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards").as_secs() as i32;
    t ^ (t >> 1)
}