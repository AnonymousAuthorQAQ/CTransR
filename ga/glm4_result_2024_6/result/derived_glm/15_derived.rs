use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter radius r and parameter p for SquareK: ");
    io::stdin().read_line(&mut input).unwrap();
    let params: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let r = params[0];
    let p = params[1];

    let square = SquareK { r, p };
    println!(
        "SquareK isInside: {}",
        if square.is_inside() { "Yes" } else { "No" }
    );
    println!(
        "SquareK overlap: {}",
        if square.overlap() { "Yes" } else { "No" }
    );

    println!("Enter radius r and parameter p for CubeK: ");
    io::stdin().read_line(&mut input).unwrap();
    let params: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let r = params[0];
    let p = params[1];

    let cube = CubeK { r, p };
    println!(
        "CubeK isInside: {}",
        if cube.is_inside() { "Yes" } else { "No" }
    );
    println!(
        "CubeK overlap: {}",
        if cube.overlap() { "Yes" } else { "No" }
    );
}

trait KShape {
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

struct SquareK {
    r: f64,
    p: f64,
}

impl KShape for SquareK {
    fn is_inside(&self) -> bool {
        self.r.powi(2) <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

struct CubeK {
    r: f64,
    p: f64,
}

impl KShape for CubeK {
    fn is_inside(&self) -> bool {
        self.r.powi(3) <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}