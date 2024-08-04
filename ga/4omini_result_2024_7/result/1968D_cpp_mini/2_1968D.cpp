use std::io::{self, BufRead};

// Function to calculate the maximum score for a player starting from given position
fn calculate(p: &[usize], a: &[i64], start_pos: usize, k: usize) -> i64 {
    let n = p.len();
    let mut visited = vec![false; n]; // Track visited nodes
    let mut max_score = 0;
    let mut current_score = 0;
    let mut steps = k;
    let mut pos = start_pos;

    // Traverse the graph until we run out of allowed steps or visit all nodes
    while !visited[pos] && steps > 0 {
        visited[pos] = true;
        max_score = max_score.max(current_score + (steps as i64) * a[pos]); // Calculate max score
        current_score += a[pos]; // Update current score
        pos = p[pos]; // Move to next position
        steps -= 1; // Decrease the remaining steps
    }

    max_score
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut iter = handle.lines();

    let t: usize = iter.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line = iter.next().unwrap().unwrap();
        let mut input = line.split_whitespace();
        let n: usize = input.next().unwrap().parse().unwrap();
        let k: usize = input.next().unwrap().parse().unwrap();
        let ps1: usize = input.next().unwrap().parse().unwrap() - 1; // Convert to 0-based index
        let ps2: usize = input.next().unwrap().parse().unwrap() - 1; // Convert to 0-based index

        let mut p = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);

        // Read the p array
        let line = iter.next().unwrap().unwrap();
        for value in line.split_whitespace() {
            p.push(value.parse::<usize>().unwrap() - 1); // Convert to 0-based index
        }

        // Read the a array
        let line = iter.next().unwrap().unwrap();
        for value in line.split_whitespace() {
            a.push(value.parse::<i64>().unwrap());
        }

        let ans1 = calculate(&p, &a, ps1, k);
        let ans2 = calculate(&p, &a, ps2, k);

        // Determine the result based on the scores
        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}