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
    fn new(r: f64, p: f64) -> Self {
        Self { r, p }
    }
}

impl AbstractK for K2D {
    fn overlap(&self) -> bool {
        self.r > self.p
    }

    fn is_inside(&self) -> bool {
        false // Defined in specific classes
    }
}

struct K3D {
    r: f64,
    p: f64,
}

impl K3D {
    fn new(r: f64, p: f64) -> Self {
        Self { r, p }
    }
}

impl AbstractK for K3D {
    fn overlap(&self) -> bool {
        self.r > self.p
    }

    fn is_inside(&self) -> bool {
        false // Defined in specific classes
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
        Self { k3d: K3D::new(r, p) }
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
    let mut input = String::new();
    
    // SquareK
    println!("Enter radius r and parameter p for SquareK: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Not a valid number")).collect();
    
    let square = SquareK::new(parts[0], parts[1]);
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });
    
    input.clear(); // Clear input for the next set of values

    // CubeK
    println!("Enter radius r and parameter p for CubeK: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Not a valid number")).collect();

    let cube = CubeK::new(parts[0], parts[1]);
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}