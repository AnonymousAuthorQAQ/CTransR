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
        false // To be defined in more specific classes like SquareK
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
        false // To be defined in more specific classes like CubeK
    }
}

struct SquareK {
    k2d: K2D,
}

impl SquareK {
    fn new(r: f64, p: f64) -> SquareK {
        SquareK {
            k2d: K2D { r, p },
        }
    }
}

impl AbstractK for SquareK {
    fn is_inside(&self) -> bool {
        self.k2d.r.powi(2) <= self.k2d.p
    }
    
    fn overlap(&self) -> bool {
        self.k2d.overlap()
    }
}

struct CubeK {
    k3d: K3D,
}

impl CubeK {
    fn new(r: f64, p: f64) -> CubeK {
        CubeK {
            k3d: K3D { r, p },
        }
    }
}

impl AbstractK for CubeK {
    fn is_inside(&self) -> bool {
        self.k3d.r.powi(3) <= self.k3d.p
    }
    
    fn overlap(&self) -> bool {
        self.k3d.overlap()
    }
}

fn main() {
    let mut r_input = String::new();
    let mut p_input = String::new();

    // Input for SquareK
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut r_input).unwrap();
    io::stdin().read_line(&mut p_input).unwrap();
    let r: f64 = r_input.trim().parse().expect("Invalid input for r");
    let p: f64 = p_input.trim().parse().expect("Invalid input for p");
    
    let square = SquareK::new(r, p);
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    r_input.clear();
    p_input.clear();

    // Input for CubeK
    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut r_input).unwrap();
    io::stdin().read_line(&mut p_input).unwrap();
    let r: f64 = r_input.trim().parse().expect("Invalid input for r");
    let p: f64 = p_input.trim().parse().expect("Invalid input for p");
    
    let cube = CubeK::new(r, p);
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}