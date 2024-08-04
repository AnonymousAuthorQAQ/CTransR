use std::io;

fn main() {
    loop {
        println!("Enter radius r and parameter p for SquareK: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let values: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Invalid number")).collect();
        let (r, p) = (values[0], values[1]);
        let square = SquareK { r, p };
        println!("SquareK isInside: {}", square.is_inside());
        println!("SquareK overlap: {}", square.overlap());

        println!("Enter radius r and parameter p for CubeK: ");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let values: Vec<f64> = input.trim().split_whitespace().map(|s| s.parse().expect("Invalid number")).collect();
        let (r, p) = (values[0], values[1]);
        let cube = CubeK { r, p };
        println!("CubeK isInside: {}", cube.is_inside());
        println!("CubeK overlap: {}", cube.overlap());
    }
}

trait AbstractK {
    fn new(r: f64, p: f64) -> Self;
    fn is_inside(&self) -> bool;
    fn overlap(&self) -> bool {
        self.r > self.p
    }
}

struct SquareK {
    r: f64,
    p: f64,
}

impl AbstractK for SquareK {
    fn new(r: f64, p: f64) -> Self {
        SquareK { r, p }
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
    fn new(r: f64, p: f64) -> Self {
        CubeK { r, p }
    }

    fn is_inside(&self) -> bool {
        self.r * self.r * self.r <= self.p
    }
}