use std::io;

fn main() {
    let mut input = String::new();

    // SquareK
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Please type a number")).collect();
    let r = values[0];
    let p = values[1];
    let square = SquareK { r, p };
    println!("SquareK isInside: {}", square.is_inside());
    println!("SquareK overlap: {}", square.overlap());

    // CubeK
    println!("Enter radius r and parameter p for CubeK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Please type a number")).collect();
    let r = values[0];
    let p = values[1];
    let cube = CubeK { r, p };
    println!("CubeK isInside: {}", cube.is_inside());
    println!("CubeK overlap: {}", cube.overlap());
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