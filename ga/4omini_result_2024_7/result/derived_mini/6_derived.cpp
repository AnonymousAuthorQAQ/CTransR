use std::io;

trait AbstractK {
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
    fn r(&self) -> f64;
    fn p(&self) -> f64;
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
    fn is_inside(&self) -> bool {
        false // To be defined in more specific classes
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }

    fn r(&self) -> f64 { self.r }
    
    fn p(&self) -> f64 { self.p }
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
    fn is_inside(&self) -> bool {
        false // To be defined in more specific classes
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }

    fn r(&self) -> f64 { self.r }
    
    fn p(&self) -> f64 { self.p }
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
    fn is_inside(&self) -> bool {
        self.k2d.r * self.k2d.r <= self.k2d.p
    }

    fn overlap(&self) -> bool {
        self.k2d.overlap()
    }

    fn r(&self) -> f64 { self.k2d.r() }
    
    fn p(&self) -> f64 { self.k2d.p() }
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
    fn is_inside(&self) -> bool {
        self.k3d.r * self.k3d.r * self.k3d.r <= self.k3d.p
    }

    fn overlap(&self) -> bool {
        self.k3d.overlap()
    }

    fn r(&self) -> f64 { self.k3d.r() }

    fn p(&self) -> f64 { self.k3d.p() }
}

fn main() {
    let mut input = String::new();

    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let values: Vec<f64> = input.trim().split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    let square_k = SquareK::new(values[0], values[1]);
    println!("SquareK isInside: {}", if square_k.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square_k.overlap() { "Yes" } else { "No" });
    input.clear();

    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let values: Vec<f64> = input.trim().split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    let cube_k = CubeK::new(values[0], values[1]);
    println!("CubeK isInside: {}", if cube_k.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube_k.overlap() { "Yes" } else { "No" });
}