use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n = parts.next().unwrap().parse::<i32>().unwrap();
        let kl = parts.next().unwrap().parse::<i32>().unwrap();
        let kr = parts.next().unwrap().parse::<i32>().unwrap();
        let cc: Vec<char> = parts.next().unwrap().chars().collect();

        let zz = compute_zigzag(&cc, n);
        let ans = compute_answers(&zz, kl, kr);
        
        for k in kl..=kr {
            println!("{} ", ans[k as usize]);
        }
    }
}

fn compute_zigzag(cc: &Vec<char>, n: i32) -> Vec<i32> {
    let mut zz = vec![0; n as usize];
    let mut l = 0;
    let mut r = 0;

    for i in 1..n {
        if i as usize + zz[i as usize - l] < r {
            zz[i as usize] = zz[i as usize - l];
        } else {
            l = i;
            r = std::cmp::max(r, i);
            
            while r < n && cc[r as usize] == cc[(r - l) as usize] {
                r += 1;
            }

            zz[i as usize] = r - l;
        }
    }

    zz
}

fn compute_answers(zz: &Vec<i32>, kl: i32, kr: i32) -> Vec<i32> {
    let mut ans = vec![0; (kr - kl + 1) as usize];

    for p in 1..=kr {
        let mut k = 0;
        let mut i = 0;

        while i < zz.len() {
            i = segment_tree_query(zz, i as i32, p);
            k += 1;
        }

        ans[(k - kl) as usize] = std::cmp::max(ans[(k - kl) as usize], p);
    }

    ans
}

fn segment_tree_query(zz: &Vec<i32>, mut l: i32, a: i32) -> i32 {
    let mut r = zz.len() as i32 - 1;

    while l <= r {
        if (l & 1) == 1 {
            if zz[l as usize] >= a {
                while l < zz.len() as i32 {
                    l = if zz[(l << 1) as usize] >= a {
                        (l << 1) as i32
                    } else {
                        (l << 1 | 1) as i32
                    };
                }
                return l;
            }
            l += 1;
        }
        l >>= 1;
        r >>= 1;
    }

    return zz.len() as i32;
}