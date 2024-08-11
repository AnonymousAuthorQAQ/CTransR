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
    fn new(r: f64, p: f64) -> K2D {
        K2D { r, p }
    }
}

impl AbstractK for K2D {
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

impl K3D {
    fn new(r: f64, p: f64) -> K3D {
        K3D { r, p }
    }
}

impl AbstractK for K3D {
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

impl SquareK {
    fn new(r: f64, p: f64) -> SquareK {
        SquareK {k2d: K2D::new(r, p)}
    }
}

impl AbstractK for SquareK {
    fn is_inside(&self) -> bool {
        self.k2d.r * self.k2d.r <= self.k2d.p
    }

    fn overlap(&self) -> bool {
        self.k2d.r > self.k2d.p
    }
}

struct CubeK {
    k3d: K3D,
}

impl CubeK {
    fn new(r: f64, p: f64) -> CubeK {
        CubeK {k3d: K3D::new(r, p)}
    }
}

impl AbstractK for CubeK {
    fn is_inside(&self) -> bool {
        self.k3d.r * self.k3d.r * self.k3d.r <= self.k3d.p
    }

    fn overlap(&self) -> bool {
        self.k3d.r > self.k3d.p
    }
}

fn main() {
    let mut input = String::new();
    let mut r: f64 = 0.0;
    let mut p: f64 = 0.0;

    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<f64> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let r1 = parts[0];
    let p1 = parts[1];
    let square = SquareK::new(r1, p1);
    println!("SquareK isInside: {}", square.is_inside());
    println!("SquareK overlap: {}", square.overlap());

    input.clear(); // Clearing previous input

    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<f64> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let r2 = parts[0];
    let p2 = parts[1];
    let cube = CubeK::new(r2, p2);
    println!("CubeK isInside: {}", cube.is_inside());
    println!("CubeK overlap: {}", cube.overlap());
}