use std::io;

trait AbstractK {
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool;
}

struct K2D {
    r: f64,
    p: f64,
}

impl AbstractK for K2D {
    fn is_inside(&self) -> bool {
        false // To be defined in more specific classes like SquareK
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

impl K2D {
    fn new(r: f64, p: f64) -> Self {
        K2D { r, p }
    }
}

struct K3D {
    r: f64,
    p: f64,
}

impl AbstractK for K3D {
    fn is_inside(&self) -> bool {
        false // To be defined in more specific classes like CubeK
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

impl K3D {
    fn new(r: f64, p: f64) -> Self {
        K3D { r, p }
    }
}

struct SquareK {
    k2d: K2D,
}

impl AbstractK for SquareK {
    fn is_inside(&self) -> bool {
        self.k2d.r.powi(2) <= self.k2d.p
    }

    fn overlap(&self) -> bool {
        self.k2d.r > self.k2d.p
    }
}

impl SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK { k2d: K2D::new(r, p) }
    }
}

struct CubeK {
    k3d: K3D,
}

impl AbstractK for CubeK {
    fn is_inside(&self) -> bool {
        self.k3d.r.powi(3) <= self.k3d.p
    }

    fn overlap(&self) -> bool {
        self.k3d.r > self.k3d.p
    }
}

impl CubeK {
    fn new(r: f64, p: f64) -> Self {
        CubeK { k3d: K3D::new(r, p) }
    }
}

fn main() {
    let mut input = String::new();
    let mut r: f64;
    let mut p: f64;

    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).unwrap();
    let values: Vec<f64> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    r = values[0];
    p = values[1];
    let square_k = SquareK::new(r, p);
    println!("SquareK isInside: {}", if square_k.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square_k.overlap() { "Yes" } else { "No" });

    input.clear();

    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).unwrap();
    let values: Vec<f64> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    r = values[0];
    p = values[1];
    let cube_k = CubeK::new(r, p);
    println!("CubeK isInside: {}", if cube_k.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube_k.overlap() { "Yes" } else { "No" });
}