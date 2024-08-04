use std::io;

fn main() {
    let mut input = String::new();

    // SquareK
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let r: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let p: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let square = SquareK { r, p };
    println!("SquareK isInside: {}", square.is_inside());
    println!("SquareK overlap: {}", square.overlap());

    // CubeK
    println!("Enter radius r and parameter p for CubeK:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    iter = input.split_whitespace();
    let r: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let p: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let cube = CubeK { r, p };
    println!("CubeK isInside: {}", cube.is_inside());
    println!("CubeK overlap: {}", cube.overlap());
}

trait AbstractK {
    fn new(r: f64, p: f64) -> Self;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

struct K2D {
    r: f64,
    p: f64,
}

impl AbstractK for K2D {
    fn new(r: f64, p: f64) -> Self {
        K2D { r, p }
    }
    fn is_inside(&self) -> bool {
        false
    }
}

struct K3D {
    r: f64,
    p: f64,
}

impl AbstractK for K3D {
    fn new(r: f64, p: f64) -> Self {
        K3D { r, p }
    }
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
}

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
}