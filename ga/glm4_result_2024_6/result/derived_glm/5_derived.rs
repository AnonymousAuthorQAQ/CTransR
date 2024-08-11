use std::io;
use std::cmp::Ordering;

trait AbstractK {
    fn new(r: f64, p: f64) -> Self;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

trait K2D: AbstractK {
    fn new(r: f64, p: f64) -> Self;
}

trait K3D: AbstractK {
    fn new(r: f64, p: f64) -> Self;
}

struct SquareK {
    r: f64,
    p: f64,
}

impl K2D for SquareK {
    fn new(r: f64, p: f64) -> Self {
        Self { r, p }
    }
}

impl AbstractK for SquareK {
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

impl K3D for CubeK {
    fn new(r: f64, p: f64) -> Self {
        Self { r, p }
    }
}

impl AbstractK for CubeK {
    fn is_inside(&self) -> bool {
        self.r.powi(3) <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter radius r and parameter p for SquareK: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut parts = input.trim().split_whitespace();
    let r: f64 = parts.next().unwrap().parse().expect("Invalid input");
    let p: f64 = parts.next().unwrap().parse().expect("Invalid input");
    let square = SquareK::new(r, p);
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    println!("Enter radius r and parameter p for CubeK: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut parts = input.trim().split_whitespace();
    let r: f64 = parts.next().unwrap().parse().expect("Invalid input");
    let p: f64 = parts.next().unwrap().parse().expect("Invalid input");
    let cube = CubeK::new(r, p);
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}