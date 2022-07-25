use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut points = Point::import();
    
}

#[derive(Debug, Clone, Copy)]
struct Point {
    id: usize,
    x: i32,
    y: i32,
}

impl Point {
    fn new(id: usize, x: i32, y: i32) -> Point {
        Point { id, x, y }
    }

    fn distance(&self, other: &Point) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }

    fn import() -> Vec<Point> {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let n = parse_input!(input_line, i32);
        let mut points = Vec::new();
        for i in 0..n as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], i32);
            let y = parse_input!(inputs[1], i32);
            points.push(Point::new(i, x, y));
        }
        points
    }
}
