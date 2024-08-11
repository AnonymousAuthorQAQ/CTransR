use std::io;

trait AbstractK {
    fn radius(&self) -> f64;
    fn parameter(&self) -> f64;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

struct K2D {
    r: f64,
    p: f64,
}

impl K2D {
    fn new(r: f64, p: f64) -> Self {
        K2D { r, p }
    }
}

impl AbstractK for K2D {
    fn radius(&self) -> f64 {
        self.r
    }

    fn parameter(&self) -> f64 {
        self.p
    }

    fn is_inside(&self) -> bool {
        false // To be implemented in SquareK
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

struct K3D {
    r: f64,
    p: f64,
}

impl K3D {
    fn new(r: f64, p: f64) -> Self {
        K3D { r, p }
    }
}

impl AbstractK for K3D {
    fn radius(&self) -> f64 {
        self.r
    }

    fn parameter(&self) -> f64 {
        self.p
    }

    fn is_inside(&self) -> bool {
        false // To be implemented in CubeK
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

struct SquareK {
    k2d: K2D,
}

impl SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK { k2d: K2D::new(r, p) }
    }
}

impl AbstractK for SquareK {
    fn radius(&self) -> f64 {
        self.k2d.radius()
    }

    fn parameter(&self) -> f64 {
        self.k2d.parameter()
    }

    fn is_inside(&self) -> bool {
        self.k2d.radius().powi(2) <= self.k2d.parameter() // Check if r^2 <= p
    }

    fn overlap(&self) -> bool {
        self.k2d.overlap()
    }
}

struct CubeK {
    k3d: K3D,
}

impl CubeK {
    fn new(r: f64, p: f64) -> Self {
        CubeK { k3d: K3D::new(r, p) }
    }
}

impl AbstractK for CubeK {
    fn radius(&self) -> f64 {
        self.k3d.radius()
    }

    fn parameter(&self) -> f64 {
        self.k3d.parameter()
    }

    fn is_inside(&self) -> bool {
        self.k3d.radius().powi(3) <= self.k3d.parameter() // Check if r^3 <= p
    }

    fn overlap(&self) -> bool {
        self.k3d.overlap()
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter radius r and parameter p for SquareK: ");
    io::stdin().read_line(&mut input).unwrap();
    let values: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let square = SquareK::new(values[0], values[1]);
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    input.clear(); // clear the input buffer

    println!("Enter radius r and parameter p for CubeK: ");
    io::stdin().read_line(&mut input).unwrap();
    let values: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let cube = CubeK::new(values[0], values[1]);
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}