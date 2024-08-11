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
        false // To be defined in more specific classes
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
    fn is_inside(&self) -> bool {
        false // To be defined in more specific classes
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

struct SquareK {
    k2d: K2D,
}

impl SquareK {
    fn new(r: f64, p: f64) -> Self {
        Self {
            k2d: K2D { r, p },
        }
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
        Self {
            k3d: K3D { r, p },
        }
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
    let mut r_input = String::new();
    let mut p_input = String::new();

    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut r_input).expect("Failed to read line");
    io::stdin().read_line(&mut p_input).expect("Failed to read line");

    let r: f64 = r_input.trim().parse().expect("Please enter a valid number");
    let p: f64 = p_input.trim().parse().expect("Please enter a valid number");

    let square = SquareK::new(r, p);
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    r_input.clear();
    p_input.clear();

    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut r_input).expect("Failed to read line");
    io::stdin().read_line(&mut p_input).expect("Failed to read line");

    let r: f64 = r_input.trim().parse().expect("Please enter a valid number");
    let p: f64 = p_input.trim().parse().expect("Please enter a valid number");

    let cube = CubeK::new(r, p);
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}