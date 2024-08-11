use std::io::{self, Read};

fn calculate(p: &Vec<usize>, a: &Vec<i64>, pos: usize, k: i64) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];

    let mut position = pos;
    let mut steps = k;
    while !vis[position] && steps > 0 {
        vis[position] = true;
        mx = mx.max(cur + (steps * a[position] as i64));
        cur += a[position] as i64;
        position = p[position];
        steps -= 1;
    }
    mx
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut lines = input.trim().split('\n');
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
      let mut line = lines.next().unwrap().split_whitespace();
      let n: usize = line.next().unwrap().parse().unwrap();
      let k: i64 = line.next().unwrap().parse().unwrap();
      let ps1: usize = line.next().unwrap().parse().unwrap();
      let ps2: usize = line.next().unwrap().parse().unwrap();

      let mut p = vec![0; n];
      for i in 0..n {
          p[i] = line.next().unwrap().parse::<usize>().unwrap() - 1;
      }

      let mut a = Vec::with_capacity(n);
      for _ in 0..n {
          a.push(line.next().unwrap().parse().unwrap());
      }

      let ans1 = calculate(&p, &a, ps1 - 1, k);
      let ans2 = calculate(&p, &a, ps2 - 1, k);

      if ans1 == ans2 {
          println!("Draw");
      } else if ans1 > ans2 {
          println!("Bodya");
      } else {
          println!("Sasha");
      }
    }
}