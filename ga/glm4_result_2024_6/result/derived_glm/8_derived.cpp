use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter radius r and parameter p for SquareK: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let params: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Invalid input")).collect();
    let r = params[0];
    let p = params[1];
    let square = SquareK { r, p };
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    println!("Enter radius r and parameter p for CubeK: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let params: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Invalid input")).collect();
    let r = params[0];
    let p = params[1];
    let cube = CubeK { r, p };
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}

trait AbstractK {
    fn new(r: f64, p: f64) -> Self;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

struct SquareK {
    r: f64,
    p: f64,
}

impl AbstractK for SquareK {
    fn new(r: f64, p: f64) -> Self {
        Self { r, p }
    }

    fn is_inside(&self) -> bool {
        self.r * self.r <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

struct CubeK {
    r: f64,
    p: f64,
}

impl AbstractK for CubeK {
    fn new(r: f64, p: f64) -> Self {
        Self { r, p }
    }

    fn is_inside(&self) -> bool {
        self.r * self.r * self.r <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}