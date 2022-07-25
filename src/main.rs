use std::{
    collections::{BinaryHeap, VecDeque},
    io,
};

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
    let ans = beam_search(&points, 3);
    for x in ans {
        print!("{} ", x);
    }
    println!();
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

    fn length(points: &Vec<Point>, path: &Vec<usize>) -> i32 {
        let mut length = 0.0;
        for i in 0..path.len() - 1 {
            length += points[path[i]].distance(&points[path[i + 1]]);
        }
        length as i32
    }
}

fn beam_search(points: &Vec<Point>, beam_width: usize) -> Vec<usize> {
    let mut deq = VecDeque::new();
    deq.push_back((points[0], vec![0]));

    let mut ans_heap = BinaryHeap::new();

    loop {
        if deq.is_empty() {
            break;
        }

        let (point, path) = deq.pop_front().unwrap();
        let mut heap = BinaryHeap::new();
        for i in 0..points.len() {
            if path.contains(&i) {
                continue;
            }
            heap.push((-points[i].distance(&point) as i32, i));
        }
        for _ in 0..beam_width {
            if heap.is_empty() {
                break;
            }
            let (dist, i) = heap.pop().unwrap();
            let mut new_path = path.clone();
            new_path.push(i);
            if new_path.len() == points.len() {
                new_path.push(0);
                ans_heap.push((-Point::length(points, &new_path), new_path));
            } else {
                deq.push_back((points[i], new_path));
            }
        }
    }

    let ans = ans_heap.pop().unwrap();

    return ans.1;
}
