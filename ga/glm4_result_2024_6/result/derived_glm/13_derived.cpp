use std::io;
use std::cmp::Ordering;

trait AbstractK {
    fn new(r: f64, p: f64) -> Self;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool {
        self.r > self.p
    }
    fn r(&self) -> f64;
    fn p(&self) -> f64;
}

struct K2D {
    r: f64,
    p: f64,
}

impl AbstractK for K2D {
    fn new(r: f64, p: f64) -> K2D {
        K2D { r, p }
    }

    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
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
    fn new(r: f64, p: f64) -> K3D {
        K3D { r, p }
    }

    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }

    fn is_inside(&self) -> bool {
        false
    }
}

struct SquareK {
    base: K2D,
}

impl AbstractK for SquareK {
    fn new(r: f64, p: f64) -> SquareK {
        SquareK {
            base: K2D::new(r, p),
        }
    }

    fn r(&self) -> f64 {
        self.base.r()
    }

    fn p(&self) -> f64 {
        self.base.p()
    }

    fn is_inside(&self) -> bool {
        self.r().powi(2) <= self.p()
    }
}

struct CubeK {
    base: K3D,
}

impl AbstractK for CubeK {
    fn new(r: f64, p: f64) -> CubeK {
        CubeK {
            base: K3D::new(r, p),
        }
    }

    fn r(&self) -> f64 {
        self.base.r()
    }

    fn p(&self) -> f64 {
        self.base.p()
    }

    fn is_inside(&self) -> bool {
        self.r().powi(3) <= self.p()
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Invalid input")).collect();
    let r = values[0];
    let p = values[1];
    let square = SquareK::new(r, p);
    println!("SquareK isInside: {}", square.is_inside());
    println!("SquareK overlap: {}", square.overlap());

    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Invalid input")).collect();
    let r = values[0];
    let p = values[1];
    let cube = CubeK::new(r, p);
    println!("CubeK isInside: {}", cube.is_inside());
    println!("CubeK overlap: {}", cube.overlap());
}