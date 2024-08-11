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
        Self { r, p }
    }
}

struct K2D {
    abstract_k: AbstractK,
}

impl K2D {
    fn new(r: f64, p: f64) -> Self {
        Self {
            abstract_k: AbstractK::new(r, p),
        }
    }
}

impl Shape for K2D {
    fn overlap(&self) -> bool {
        self.abstract_k.r > self.abstract_k.p
    }

    fn is_inside(&self) -> bool {
        false // To be defined more specifically in derived structs like SquareK
    }
}

struct K3D {
    abstract_k: AbstractK,
}

impl K3D {
    fn new(r: f64, p: f64) -> Self {
        Self {
            abstract_k: AbstractK::new(r, p),
        }
    }
}

impl Shape for K3D {
    fn overlap(&self) -> bool {
        self.abstract_k.r > self.abstract_k.p
    }

    fn is_inside(&self) -> bool {
        false // To be defined more specifically in derived structs like CubeK
    }
}

struct SquareK {
    k2d: K2D,
}

impl SquareK {
    fn new(r: f64, p: f64) -> Self {
        Self { k2d: K2D::new(r, p) }
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

struct CubeK {
    k3d: K3D,
}

impl CubeK {
    fn new(r: f64, p: f64) -> Self {
        Self { k3d: K3D::new(r, p) }
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

    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<f64> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Not a number"))
        .collect();
    let square = SquareK::new(values[0], values[1]);
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    input.clear();
    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<f64> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Not a number"))
        .collect();
    let cube = CubeK::new(values[0], values[1]);
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}