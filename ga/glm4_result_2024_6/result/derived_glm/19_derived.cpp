use std::io;
use std::cmp::Ordering;

trait AbstractK {
    fn new(r: f64, p: f64) -> Self;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

trait K2D {
    fn new(r: f64, p: f64) -> Self;
}

trait K3D {
    fn new(r: f64, p: f64) -> Self;
}

#[derive(Debug)]
struct SquareK {
    r: f64,
    p: f64,
}

impl AbstractK for SquareK {
    fn new(r: f64, p: f64) -> Self {
        Self { r, p }
    }

    fn is_inside(&self) -> bool {
        self.r.powi(2) <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

impl K2D for SquareK {
    fn new(r: f64, p: f64) -> Self {
        AbstractK::new(r, p)
    }
}

#[derive(Debug)]
struct CubeK {
    r: f64,
    p: f64,
}

impl AbstractK for CubeK {
    fn new(r: f64, p: f64) -> Self {
        Self { r, p }
    }

    fn is_inside(&self) -> bool {
        self.r.powi(3) <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

impl K3D for CubeK {
    fn new(r: f64, p: f64) -> Self {
        AbstractK::new(r, p)
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Please type a number!")).collect();
    let r = values[0];
    let p = values[1];
    let square_k = SquareK::new(r, p);
    println!("SquareK is_inside: {}", square_k.is_inside());
    println!("SquareK overlap: {}", square_k.overlap());

    println!("Enter radius r and parameter p for CubeK:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Please type a number!")).collect();
    let r = values[0];
    let p = values[1];
    let cube_k = CubeK::new(r, p);
    println!("CubeK is_inside: {}", cube_k.is_inside());
    println!("CubeK overlap: {}", cube_k.overlap());
}