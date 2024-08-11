use std::io;
use std::cmp::min;

const N: usize = 2e5 as usize + 3;
const ANS: [i32; N] = [0; N];

fn find(p: &mut [i32], mut k: i32) -> i32 {
    if k > p.len() as i32 || p[k as usize] == k {
        k
    } else {
        p[k as usize] = find(p, p[k as usize]);
        p[k as usize]
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut tokens = input.trim().split_whitespace();
        let n: i32 = tokens.next().unwrap().parse().unwrap();
        let x: i32 = tokens.next().unwrap().parse().unwrap();
        let y: i32 = tokens.next().unwrap().parse().unwrap();
        let a = tokens.next().unwrap();

        let mut p: Vec<i32> = (0..n as usize + 2).map(|i| i as i32).collect();
        let mut t: Vec<i32> = (0..n as usize + 1).map(|i| i as i32).collect();
        let mut z: Vec<i32> = vec![0; n as usize + 2];
        z[1] = n;

        for i in 2..=n {
            let mut j = 0;
            while j + z[j as usize] > i {
                z[i as usize] = min(z[i as usize - j as usize + 1], j + z[j as usize] - i);
            }
            while a.chars().nth(i as usize + z[i as usize] - 1) == a.chars().nth(z[i as usize] - 1) {
                z[i as usize] += 1;
            }
            if i + z[i as usize] > j + z[j as usize] {
                j = i;
            }
        }

        t.sort_by(|&i, &j| z[i as usize].cmp(&z[j as usize]));

        for i in 1..=n {
            let mut c = 0;
            let mut k = 1;
            while k <= n && z[t[k as usize] as usize] < i {
                p[t[k as usize] as usize] = t[k as usize] + 1;
                k += 1;
            }
            for j in 1..=n {
                c += 1;
                j = find(&mut p, j + i);
            }
            ANS[c as usize] = i;
        }

        for i in (0..n).rev() {
            ANS[i as usize] = ANS[i as usize + 1].max(ANS[i as usize]);
        }

        for i in x..=y {
            print!("{} ", ANS[i as usize]);
        }
        println!();
    }
}