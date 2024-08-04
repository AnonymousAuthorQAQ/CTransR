use std::io;
use std::cmp::Ordering;

trait AbstractK {
    fn new(r: f64, p: f64) -> Self;
    fn r(&self) -> f64;
    fn p(&self) -> f64;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

trait K2D: AbstractK {
    fn is_inside(&self) -> bool {
        false // To be defined in more specific classes like SquareK
    }
}

trait K3D: AbstractK {
    fn is_inside(&self) -> bool {
        false // To be defined in more specific classes like CubeK
    }
}

struct SquareK {
    r: f64,
    p: f64,
}

impl AbstractK for SquareK {
    fn new(r: f64, p: f64) -> Self {
        Self { r, p }
    }

    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

impl K2D for SquareK {
    fn is_inside(&self) -> bool {
        self.r * self.r <= self.p
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

    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

impl K3D for CubeK {
    fn is_inside(&self) -> bool {
        self.r * self.r * self.r <= self.p
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.trim().split_whitespace();
    let r: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let p: f64 = iter.next().unwrap().parse().expect("Invalid input");

    let square = SquareK { r, p };
    println!("SquareK isInside: {}", square.is_inside());
    println!("SquareK overlap: {}", square.overlap());

    input.clear();
    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    iter = input.trim().split_whitespace();
    let r: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let p: f64 = iter.next().unwrap().parse().expect("Invalid input");

    let cube = CubeK { r, p };
    println!("CubeK isInside: {}", cube.is_inside());
    println!("CubeK overlap: {}", cube.overlap());
}