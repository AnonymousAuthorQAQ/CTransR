use std::io;
use std::cmp::Ordering;

trait AbstractK {
    fn new(r: f64, p: f64) -> Self;
    fn r(&self) -> f64;
    fn p(&self) -> f64;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

struct K2D {
    r: f64,
    p: f64,
}

impl AbstractK for K2D {
    fn new(r: f64, p: f64) -> Self {
        K2D { r, p }
    }

    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }

    fn is_inside(&self) -> bool {
        false // To be defined in more specific structs like SquareK
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

struct K3D {
    r: f64,
    p: f64,
}

impl AbstractK for K3D {
    fn new(r: f64, p: f64) -> Self {
        K3D { r, p }
    }

    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }

    fn is_inside(&self) -> bool {
        false // To be defined in more specific structs like CubeK
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

struct SquareK {
    k2d: K2D,
}

impl AbstractK for SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK { k2d: K2D::new(r, p) }
    }

    fn r(&self) -> f64 {
        self.k2d.r()
    }

    fn p(&self) -> f64 {
        self.k2d.p()
    }

    fn is_inside(&self) -> bool {
        self.r.powi(2) <= self.p
    }

    fn overlap(&self) -> bool {
        self.k2d.overlap()
    }
}

struct CubeK {
    k3d: K3D,
}

impl AbstractK for CubeK {
    fn new(r: f64, p: f64) -> Self {
        CubeK { k3d: K3D::new(r, p) }
    }

    fn r(&self) -> f64 {
        self.k3d.r()
    }

    fn p(&self) -> f64 {
        self.k3d.p()
    }

    fn is_inside(&self) -> bool {
        self.r.powi(3) <= self.p
    }

    fn overlap(&self) -> bool {
        self.k3d.overlap()
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter radius r and parameter p for SquareK: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let r_square: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let p_square: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let square = SquareK::new(r_square, p_square);
    println!("SquareK isInside: {}", square.is_inside());
    println!("SquareK overlap: {}", square.overlap());

    println!("Enter radius r and parameter p for CubeK: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let r_cube: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let p_cube: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let cube = CubeK::new(r_cube, p_cube);
    println!("CubeK isInside: {}", cube.is_inside());
    println!("CubeK overlap: {}", cube.overlap());
}