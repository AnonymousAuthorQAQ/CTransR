use std::io;

trait AbstractK {
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

struct K2D {
    r: f64,
    p: f64,
}

impl AbstractK for K2D {
    fn is_inside(&self) -> bool {
        false
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

impl K2D {
    fn new(r: f64, p: f64) -> Self {
        K2D { r, p }
    }
}

struct K3D {
    r: f64,
    p: f64,
}

impl AbstractK for K3D {
    fn is_inside(&self) -> bool {
        false
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

impl K3D {
    fn new(r: f64, p: f64) -> Self {
        K3D { r, p }
    }
}

struct SquareK {
    r: f64,
    p: f64,
}

impl AbstractK for SquareK {
    fn is_inside(&self) -> bool {
        self.r * self.r <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

impl SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK { r, p }
    }
}

struct CubeK {
    r: f64,
    p: f64,
}

impl AbstractK for CubeK {
    fn is_inside(&self) -> bool {
        self.r * self.r * self.r <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

impl CubeK {
    fn new(r: f64, p: f64) -> Self {
        CubeK { r, p }
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let values: Vec<f64> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let squarek = SquareK::new(values[0], values[1]);

    println!("SquareK isInside: {}", if squarek.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if squarek.overlap() { "Yes" } else { "No" });

    input.clear();
    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let values: Vec<f64> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let cubek = CubeK::new(values[0], values[1]);

    println!("CubeK isInside: {}", if cubek.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cubek.overlap() { "Yes" } else { "No" });
}