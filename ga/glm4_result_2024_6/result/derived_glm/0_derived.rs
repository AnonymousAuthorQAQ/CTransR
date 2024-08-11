use std::io;

fn main() {
    let mut input = String::new();

    // Input for SquareK
    println!("Enter radius r and parameter p for SquareK:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.trim().split_whitespace();
    let r_square: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let p_square: f64 = iter.next().unwrap().parse().expect("Invalid input");

    let square = SquareK { r: r_square, p: p_square };
    println!("SquareK isInside: {}", square.is_inside());
    println!("SquareK overlap: {}", square.overlap());

    // Input for CubeK
    println!("Enter radius r and parameter p for CubeK:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    iter = input.trim().split_whitespace();
    let r_cube: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let p_cube: f64 = iter.next().unwrap().parse().expect("Invalid input");

    let cube = CubeK { r: r_cube, p: p_cube };
    println!("CubeK isInside: {}", cube.is_inside());
    println!("CubeK overlap: {}", cube.overlap());
}

trait K {
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

impl K for SquareK {
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

impl K for CubeK {
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