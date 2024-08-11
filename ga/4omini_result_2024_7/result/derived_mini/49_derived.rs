use std::io;

trait Shape {
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

struct AbstractK {
    r: f64,
    p: f64,
}

impl AbstractK {
    fn new(r: f64, p: f64) -> Self {
        AbstractK { r, p }
    }
}

struct K2D {
    abstract_k: AbstractK,
}

impl K2D {
    fn new(r: f64, p: f64) -> Self {
        K2D { abstract_k: AbstractK::new(r, p) }
    }
}

impl Shape for K2D {
    fn overlap(&self) -> bool {
        self.abstract_k.r > self.abstract_k.p
    }

    fn is_inside(&self) -> bool {
        false  // To be defined in derived structs
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

impl Shape for SquareK {
    fn is_inside(&self) -> bool {
        self.k2d.abstract_k.r * self.k2d.abstract_k.r <= self.k2d.abstract_k.p
    }

    fn overlap(&self) -> bool {
        self.k2d.overlap()
    }
}

struct K3D {
    abstract_k: AbstractK,
}

impl K3D {
    fn new(r: f64, p: f64) -> Self {
        K3D { abstract_k: AbstractK::new(r, p) }
    }
}

impl Shape for K3D {
    fn overlap(&self) -> bool {
        self.abstract_k.r > self.abstract_k.p
    }

    fn is_inside(&self) -> bool {
        false  // To be defined in derived structs
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

impl Shape for CubeK {
    fn is_inside(&self) -> bool {
        self.k3d.abstract_k.r * self.k3d.abstract_k.r * self.k3d.abstract_k.r <= self.k3d.abstract_k.p
    }

    fn overlap(&self) -> bool {
        self.k3d.overlap()
    }
}

fn main() {
    let mut input = String::new();

    // SquareK input and evaluation
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).unwrap();
    let params: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let square = SquareK::new(params[0], params[1]);

    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    input.clear();

    // CubeK input and evaluation
    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).unwrap();
    let params: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let cube = CubeK::new(params[0], params[1]);

    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}