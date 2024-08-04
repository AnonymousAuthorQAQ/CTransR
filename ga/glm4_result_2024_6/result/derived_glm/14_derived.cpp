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
        false
    }
    fn overlap(&self) -> bool {
        self.r() > self.p()
    }
}

trait K3D: AbstractK {
    fn is_inside(&self) -> bool {
        false
    }
    fn overlap(&self) -> bool {
        self.r() > self.p()
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

    fn is_inside(&self) -> bool {
        self.r * self.r <= self.p
    }
}

impl K2D for SquareK {}

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

    fn is_inside(&self) -> bool {
        self.r * self.r * self.r <= self.p
    }
}

impl K3D for CubeK {}

fn main() {
    loop {
        println!("Enter radius r and parameter p for SquareK:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let values: Vec<f64> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<f64>().expect("Please enter a number"))
            .collect();
        if let [r, p] = values.as_slice() {
            let square = SquareK::new(*r, *p);
            println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
            println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });
        } else {
            println!("Please enter two numbers.");
            continue;
        }

        println!("Enter radius r and parameter p for CubeK:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let values: Vec<f64> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<f64>().expect("Please enter a number"))
            .collect();
        if let [r, p] = values.as_slice() {
            let cube = CubeK::new(*r, *p);
            println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
            println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
        } else {
            println!("Please enter two numbers.");
            continue;
        }

        break;
    }
}