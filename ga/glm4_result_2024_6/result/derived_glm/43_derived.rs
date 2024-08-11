use std::io;
use std::cmp::Ordering;

trait AbstractK {
    fn new(r: f64, p: f64) -> Self;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

#[derive(Debug)]
struct K2D {
    r: f64,
    p: f64,
}

impl AbstractK for K2D {
    fn new(r: f64, p: f64) -> Self {
        K2D { r, p }
    }

    fn is_inside(&self) -> bool {
        false // To be defined in more specific classes like SquareK
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

#[derive(Debug)]
struct K3D {
    r: f64,
    p: f64,
}

impl AbstractK for K3D {
    fn new(r: f64, p: f64) -> Self {
        K3D { r, p }
    }

    fn is_inside(&self) -> bool {
        false // To be defined in more specific classes like CubeK
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

#[derive(Debug)]
struct SquareK {
    k2d: K2D,
}

impl AbstractK for SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK { k2d: K2D::new(r, p) }
    }

    fn is_inside(&self) -> bool {
        self.k2d.r * self.k2d.r <= self.k2d.p
    }

    fn overlap(&self) -> bool {
        self.k2d.overlap()
    }
}

#[derive(Debug)]
struct CubeK {
    k3d: K3D,
}

impl AbstractK for CubeK {
    fn new(r: f64, p: f64) -> Self {
        CubeK { k3d: K3D::new(r, p) }
    }

    fn is_inside(&self) -> bool {
        self.k3d.r * self.k3d.r * self.k3d.r <= self.k3d.p
    }

    fn overlap(&self) -> bool {
        self.k3d.overlap()
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter radius r and parameter p for SquareK: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut parts = input.trim().split_whitespace();
    let r: f64 = parts.next().unwrap().parse().expect("Invalid input");
    let p: f64 = parts.next().unwrap().parse().expect("Invalid input");

    let square = SquareK::new(r, p);
    println!("SquareK is_inside: {}", square.is_inside());
    println!("SquareK overlap: {}", square.overlap());

    println!("Enter radius r and parameter p for CubeK: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    parts = input.trim().split_whitespace();
    let r: f64 = parts.next().unwrap().parse().expect("Invalid input");
    let p: f64 = parts.next().unwrap().parse().expect("Invalid input");

    let cube = CubeK::new(r, p);
    println!("CubeK is_inside: {}", cube.is_inside());
    println!("CubeK overlap: {}", cube.overlap());
}