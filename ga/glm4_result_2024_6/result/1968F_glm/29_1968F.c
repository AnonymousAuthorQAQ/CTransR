use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{self, BufRead};
use std::cmp::Ordering;

fn main() {
    let mut stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let t = line.parse::<usize>().unwrap();

    for _ in 0..t {
        let line = stdin.lock().lines().next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        let mut aa: Vec<usize> = Vec::with_capacity(n);
        for _ in 0..n {
            let line = stdin.lock().lines().next().unwrap().unwrap();
            let a = line.parse::<usize>().unwrap();
            aa.push(a);
        }

        for _ in 0..q {
            let line = stdin.lock().lines().next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap() - 1;
            let r: usize = parts.next().unwrap().parse().unwrap() - 1;
            
            println!("{}", if is_less_than(&aa, l, r) { "YES" } else { "NO" });
        }
    }
}

fn is_less_than(aa: &Vec<usize>, l: usize, r: usize) -> bool {
    let mut sorted_indices = (0..aa.len()).collect::<Vec<usize>>();
    quicksort_indices_by_value_and_index(aa, &mut sorted_indices, 0, aa.len());

    let l_index = binary_search_lower_bound(aa, &sorted_indices, l);
    let r_index = binary_search_lower_bound(aa, &sorted_indices, r);

    l_index < r_index
}

fn quicksort_indices_by_value_and_index(aa: &Vec<usize>, sorted_indices: &mut Vec<usize>, l: usize, r: usize) {
    if r - l <= 1 {
        return;
    }

    let pivot_index = l + rand() % (r - l);
    let pivot_value = aa[sorted_indices[pivot_index]];

    let mut i = l;
    let mut j = l;
    let mut k = r;

    while j < k {
        match aa[sorted_indices[j]].cmp(&pivot_value) {
            Ordering::Less => {
                sorted_indices.swap(i, j);
                i += 1;
                j += 1;
            }
            Ordering::Equal => {
                if sorted_indices[j] < sorted_indices[pivot_index] {
                    sorted_indices.swap(j, k - 1);
                    k -= 1;
                } else {
                    j += 1;
                }
            }
            Ordering::Greater => {
                k -= 1;
            }
        }
    }

    quicksort_indices_by_value_and_index(aa, sorted_indices, l, i);
    quicksort_indices_by_value_and_index(aa, sorted_indices, j, r);
}

fn binary_search_lower_bound(aa: &Vec<usize>, sorted_indices: &Vec<usize>, i: usize) -> usize {
    let mut lower = 0;
    let mut upper = sorted_indices.len();

    while upper - lower > 1 {
        let mid = (lower + upper) / 2;

        if aa[sorted_indices[mid]] < aa[i] || (aa[sorted_indices[mid]] == aa[i] && sorted_indices[mid] < i) {
            lower = mid;
        } else {
            upper = mid;
        }
    }

    lower
}

fn rand() -> usize {
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    seed as usize
}