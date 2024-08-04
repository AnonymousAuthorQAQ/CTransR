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
        false // To be defined in more specific classes like SquareK
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
        false // To be defined in more specific classes like CubeK
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

struct SquareK {
    r: f64,
    p: f64,
}

impl SquareK {
    fn new(r: f64, p: f64) -> SquareK {
        SquareK { r, p }
    }
}

impl AbstractK for SquareK {
    fn is_inside(&self) -> bool {
        self.r.powi(2) <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

struct CubeK {
    r: f64,
    p: f64,
}

impl CubeK {
    fn new(r: f64, p: f64) -> CubeK {
        CubeK { r, p }
    }
}

impl AbstractK for CubeK {
    fn is_inside(&self) -> bool {
        self.r.powi(3) <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

fn main() {
    println!("Enter radius r and parameter p for SquareK:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let tokens: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let r = tokens[0];
    let p = tokens[1];
    let square = SquareK::new(r, p);

    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    println!("Enter radius r and parameter p for CubeK:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let tokens: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let r = tokens[0];
    let p = tokens[1];
    let cube = CubeK::new(r, p);

    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}