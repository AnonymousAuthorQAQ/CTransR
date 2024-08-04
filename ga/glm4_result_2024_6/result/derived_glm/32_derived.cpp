use std::io;
use std::cmp::Ordering;

trait AbstractK {
    fn new(r: f64, p: f64) -> Self;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
    fn r(&self) -> f64;
    fn p(&self) -> f64;
}

trait K2D: AbstractK {
    fn is_inside(&self) -> bool {
        false
    }
}

trait K3D: AbstractK {
    fn is_inside(&self) -> bool {
        false
    }
}

struct SquareK {
    r: f64,
    p: f64,
}

impl AbstractK for SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK { r, p }
    }

    fn is_inside(&self) -> bool {
        self.r.powi(2) <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }

    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }
}

impl K2D for SquareK {}

struct CubeK {
    r: f64,
    p: f64,
}

impl AbstractK for CubeK {
    fn new(r: f64, p: f64) -> Self {
        CubeK { r, p }
    }

    fn is_inside(&self) -> bool {
        self.r.powi(3) <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }

    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }
}

impl K3D for CubeK {}

fn main() {
    let mut input = String::new();

    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let r: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let p: f64 = iter.next().unwrap().parse().expect("Invalid input");

    let square = SquareK::new(r, p);
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let r: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let p: f64 = iter.next().unwrap().parse().expect("Invalid input");

    let cube = CubeK::new(r, p);
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}