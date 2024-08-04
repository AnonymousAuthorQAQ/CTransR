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
        false
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
        false
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
        self.r() * self.r() <= self.p()
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
        self.r() * self.r() * self.r() <= self.p()
    }

    fn overlap(&self) -> bool {
        self.k3d.overlap()
    }
}

fn main() {
    loop {
        println!("Enter radius r and parameter p for SquareK: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.trim().split_whitespace();
        let r: f64 = iter.next().unwrap().parse().expect("Invalid input");
        let p: f64 = iter.next().unwrap().parse().expect("Invalid input");

        let square = SquareK::new(r, p);
        println!("SquareK isInside: {}", square.is_inside());
        println!("SquareK overlap: {}", square.overlap());

        println!("Enter radius r and parameter p for CubeK: ");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        iter = input.trim().split_whitespace();
        let r: f64 = iter.next().unwrap().parse().expect("Invalid input");
        let p: f64 = iter.next().unwrap().parse().expect("Invalid input");

        let cube = CubeK::new(r, p);
        println!("CubeK isInside: {}", cube.is_inside());
        println!("CubeK overlap: {}", cube.overlap());
    }
}