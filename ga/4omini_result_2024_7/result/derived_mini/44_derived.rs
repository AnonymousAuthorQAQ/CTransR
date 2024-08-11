use std::io;

trait AbstractK {
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
    fn overlap(&self) -> bool {
        self.r > self.p
    }

    fn is_inside(&self) -> bool {
        false // To be defined in derived classes
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
    fn overlap(&self) -> bool {
        self.r > self.p
    }

    fn is_inside(&self) -> bool {
        false // To be defined in derived classes
    }
}

struct SquareK {
    base: K2D,
}

impl SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK { base: K2D::new(r, p) }
    }
}

impl AbstractK for SquareK {
    fn is_inside(&self) -> bool {
        self.base.r * self.base.r <= self.base.p
    }

    fn overlap(&self) -> bool {
        self.base.overlap()
    }
}

struct CubeK {
    base: K3D,
}

impl CubeK {
    fn new(r: f64, p: f64) -> Self {
        CubeK { base: K3D::new(r, p) }
    }
}

impl AbstractK for CubeK {
    fn is_inside(&self) -> bool {
        self.base.r.powi(3) <= self.base.p
    }

    fn overlap(&self) -> bool {
        self.base.overlap()
    }
}

fn main() {
    let mut r = String::new();
    let mut p = String::new();

    // Input for SquareK
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut r).expect("Failed to read line");
    let values: Vec<f64> = r.trim().split_whitespace()
                              .map(|s| s.parse().unwrap())
                              .collect();
    
    let square = SquareK::new(values[0], values[1]);
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    // Input for CubeK
    r.clear();
    p.clear();
    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut r).expect("Failed to read line");
    let values: Vec<f64> = r.trim().split_whitespace()
                              .map(|s| s.parse().unwrap())
                              .collect();
    
    let cube = CubeK::new(values[0], values[1]);
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}