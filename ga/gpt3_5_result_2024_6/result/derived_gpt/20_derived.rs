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
    fn new(r: f64, p: f64) -> K2D {
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
    fn new(r: f64, p: f64) -> K3D {
        K3D { r, p }
    }
}

struct SquareK {
    k2d: K2D,
}

impl AbstractK for SquareK {
    fn is_inside(&self) -> bool {
        self.k2d.r * self.k2d.r <= self.k2d.p
    }

    fn overlap(&self) -> bool {
        self.k2d.overlap()
    }
}

impl SquareK {
    fn new(r: f64, p: f64) -> SquareK {
        SquareK { k2d: K2D::new(r, p) }
    }
}

struct CubeK {
    k3d: K3D,
}

impl AbstractK for CubeK {
    fn is_inside(&self) -> bool {
        self.k3d.r * self.k3d.r * self.k3d.r <= self.k3d.p
    }

    fn overlap(&self) -> bool {
        self.k3d.overlap()
    }
}

impl CubeK {
    fn new(r: f64, p: f64) -> CubeK {
        CubeK { k3d: K3D::new(r, p) }
    }
}

fn main() {
    let mut input = String::new();
    
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let values: Vec<f64> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let r_square = values[0];
    let p_square = values[1];
    
    let square_k = SquareK::new(r_square, p_square);
    
    input.clear();
    
    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let values: Vec<f64> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let r_cube = values[0];
    let p_cube = values[1];
    
    let cube_k = CubeK::new(r_cube, p_cube);
    
    println!("SquareK isInside: {}", if square_k.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square_k.overlap() { "Yes" } else { "No" });
    println!("CubeK isInside: {}", if cube_k.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube_k.overlap() { "Yes" } else { "No" });
}