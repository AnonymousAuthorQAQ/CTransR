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
    r: f64,
    p: f64,
}

impl AbstractK for SquareK {
    fn is_inside(&self) -> bool {
        self.r * self.r <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

impl SquareK {
    fn new(r: f64, p: f64) -> SquareK {
        SquareK { r, p }
    }
}

struct CubeK {
    r: f64,
    p: f64,
}

impl AbstractK for CubeK {
    fn is_inside(&self) -> bool {
        self.r * self.r * self.r <= self.p
    }

    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

impl CubeK {
    fn new(r: f64, p: f64) -> CubeK {
        CubeK { r, p }
    }
}

fn main() {
    let mut input = String::new();
    
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let split: Vec<f64> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let r_square = split[0];
    let p_square = split[1];
    let square = SquareK::new(r_square, p_square);

    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    input.clear();

    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let split: Vec<f64> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let r_cube = split[0];
    let p_cube = split[1];
    let cube = CubeK::new(r_cube, p_cube);

    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}