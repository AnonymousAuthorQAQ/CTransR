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
    fn overlap(&self) -> bool {
        self.r > self.p
    }
    
    fn is_inside(&self) -> bool {
        false // To be defined in specific struct
    }
}

struct K3D {
    r: f64,
    p: f64,
}

impl AbstractK for K3D {
    fn overlap(&self) -> bool {
        self.r > self.p
    }
    
    fn is_inside(&self) -> bool {
        false // To be defined in specific struct
    }
}

struct SquareK {
    k2d: K2D,
}

impl SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK { k2d: K2D { r, p } }
    }
}

impl AbstractK for SquareK {
    fn is_inside(&self) -> bool {
        self.k2d.r * self.k2d.r <= self.k2d.p
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
        CubeK { k3d: K3D { r, p } }
    }
}

impl AbstractK for CubeK {
    fn is_inside(&self) -> bool {
        self.k3d.r * self.k3d.r * self.k3d.r <= self.k3d.p
    }
    
    fn overlap(&self) -> bool {
        self.k3d.overlap()
    }
}

fn main() {
    let mut r = String::new();
    let mut p = String::new();

    // Input for SquareK
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut r).expect("Failed to read line");
    let values: Vec<f64> = r.trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();
    if values.len() == 2 {
        let square = SquareK::new(values[0], values[1]);
        println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
        println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });
    } else {
        println!("Invalid input. Please enter two numbers.");
    }

    // Input for CubeK
    r.clear();
    p.clear();
    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut r).expect("Failed to read line");
    let values: Vec<f64> = r.trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();
    if values.len() == 2 {
        let cube = CubeK::new(values[0], values[1]);
        println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
        println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
    } else {
        println!("Invalid input. Please enter two numbers.");
    }
}