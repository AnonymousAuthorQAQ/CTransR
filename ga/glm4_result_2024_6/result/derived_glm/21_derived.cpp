use std::io;
use std::io::prelude::*;

// An abstract base trait for shapes
trait AbstractK {
    fn new(r: f64, p: f64) -> Self;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
    fn r(&self) -> f64;
    fn p(&self) -> f64;
}

// A 2D shape
struct K2D {
    r: f64,
    p: f64,
}

impl AbstractK for K2D {
    fn new(r: f64, p: f64) -> K2D {
        K2D { r, p }
    }

    fn is_inside(&self) -> bool {
        false
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }

    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }
}

// A 3D shape
struct K3D {
    r: f64,
    p: f64,
}

impl AbstractK for K3D {
    fn new(r: f64, p: f64) -> K3D {
        K3D { r, p }
    }

    fn is_inside(&self) -> bool {
        false
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }

    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }
}

// A 2D square shape
struct SquareK {
    k2d: K2D,
}

impl SquareK {
    fn new(r: f64, p: f64) -> SquareK {
        SquareK { k2d: K2D::new(r, p) }
    }
}

impl AbstractK for SquareK {
    fn is_inside(&self) -> bool {
        self.k2d.r() * self.k2d.r() <= self.k2d.p()
    }

    fn overlap(&self) -> bool {
        self.k2d.overlap()
    }

    fn r(&self) -> f64 {
        self.k2d.r()
    }

    fn p(&self) -> f64 {
        self.k2d.p()
    }
}

// A 3D cube shape
struct CubeK {
    k3d: K3D,
}

impl CubeK {
    fn new(r: f64, p: f64) -> CubeK {
        CubeK { k3d: K3D::new(r, p) }
    }
}

impl AbstractK for CubeK {
    fn is_inside(&self) -> bool {
        self.k3d.r() * self.k3d.r() * self.k3d.r() <= self.k3d.p()
    }

    fn overlap(&self) -> bool {
        self.k3d.overlap()
    }

    fn r(&self) -> f64 {
        self.k3d.r()
    }

    fn p(&self) -> f64 {
        self.k3d.p()
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let r: f64 = input.trim().parse().expect("Please enter a valid number");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let p: f64 = input.trim().parse().expect("Please enter a valid number");

    let square = SquareK::new(r, p);
    println!("SquareK isInside: {}", square.is_inside());
    println!("SquareK overlap: {}", square.overlap());

    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let r: f64 = input.trim().parse().expect("Please enter a valid number");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let p: f64 = input.trim().parse().expect("Please enter a valid number");

    let cube = CubeK::new(r, p);
    println!("CubeK isInside: {}", cube.is_inside());
    println!("CubeK overlap: {}", cube.overlap());
}