use std::io;

fn main() {
    let mut input = String::new();

    // Prompt for input and read the radius r and parameter p for SquareK
    println!("Enter radius r and parameter p for SquareK: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Invalid number")).collect();
    let r_square = values[0];
    let p_square = values[1];
    let square = SquareK { r: r_square, p: p_square };
    println!("SquareK isInside: {}", if square.is_inside() { "Yes" } else { "No" });
    println!("SquareK overlap: {}", if square.overlap() { "Yes" } else { "No" });

    // Prompt for input and read the radius r and parameter p for CubeK
    input.clear();
    println!("Enter radius r and parameter p for CubeK: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Invalid number")).collect();
    let r_cube = values[0];
    let p_cube = values[1];
    let cube = CubeK { r: r_cube, p: p_cube };
    println!("CubeK isInside: {}", if cube.is_inside() { "Yes" } else { "No" });
    println!("CubeK overlap: {}", if cube.overlap() { "Yes" } else { "No" });
}

trait AbstractK {
    fn r(&self) -> f64;
    fn p(&self) -> f64;
    fn is_inside(&self) -> bool {
        false
    }
    fn overlap(&self) -> bool {
        self.r() > self.p()
    }
}

struct K2D {
    r: f64,
    p: f64,
}

impl AbstractK for K2D {
    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }
}

struct K3D {
    r: f64,
    p: f64,
}

impl AbstractK for K3D {
    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }
}

struct SquareK {
    r: f64,
    p: f64,
}

impl AbstractK for SquareK {
    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }

    fn is_inside(&self) -> bool {
        self.r * self.r <= self.p
    }
}

struct CubeK {
    r: f64,
    p: f64,
}

impl AbstractK for CubeK {
    fn r(&self) -> f64 {
        self.r
    }

    fn p(&self) -> f64 {
        self.p
    }

    fn is_inside(&self) -> bool {
        self.r * self.r * self.r <= self.p
    }
}