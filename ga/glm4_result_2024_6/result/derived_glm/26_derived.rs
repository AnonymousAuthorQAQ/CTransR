use std::io;
use std::cmp::Ordering;

trait AbstractK {
    fn new(r: f64, p: f64) -> Self;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

enum K2D {
    SquareK(SquareK),
}

impl AbstractK for K2D {
    fn new(r: f64, p: f64) -> Self {
        K2D::SquareK(SquareK::new(r, p))
    }

    fn is_inside(&self) -> bool {
        match self {
            K2D::SquareK(square) => square.is_inside(),
        }
    }

    fn overlap(&self) -> bool {
        match self {
            K2D::SquareK(square) => square.overlap(),
        }
    }
}

struct SquareK {
    r: f64,
    p: f64,
}

impl SquareK {
    fn new(r: f64, p: f64) -> Self {
        Self { r, p }
    }

    fn is_inside(&self) -> bool {
        self.r * self.r <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

enum K3D {
    CubeK(CubeK),
}

impl AbstractK for K3D {
    fn new(r: f64, p: f64) -> Self {
        K3D::CubeK(CubeK::new(r, p))
    }

    fn is_inside(&self) -> bool {
        match self {
            K3D::CubeK(cube) => cube.is_inside(),
        }
    }

    fn overlap(&self) -> bool {
        match self {
            K3D::CubeK(cube) => cube.overlap(),
        }
    }
}

struct CubeK {
    r: f64,
    p: f64,
}

impl CubeK {
    fn new(r: f64, p: f64) -> Self {
        Self { r, p }
    }

    fn is_inside(&self) -> bool {
        self.r * self.r * self.r <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

fn main() {
    loop {
        println!("Enter radius r and parameter p for SquareK:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.trim().split_whitespace();
        let r: f64 = iter.next().unwrap().parse().expect("Invalid input");
        let p: f64 = iter.next().unwrap().parse().expect("Invalid input");

        let square = K2D::new(r, p);
        println!("SquareK isInside: {}", square.is_inside());
        println!("SquareK overlap: {}", square.overlap());

        println!("Enter radius r and parameter p for CubeK:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.trim().split_whitespace();
        let r: f64 = iter.next().unwrap().parse().expect("Invalid input");
        let p: f64 = iter.next().unwrap().parse().expect("Invalid input");

        let cube = K3D::new(r, p);
        println!("CubeK isInside: {}", cube.is_inside());
        println!("CubeK overlap: {}", cube.overlap());
    }
}