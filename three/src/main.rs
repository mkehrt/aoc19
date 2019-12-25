use std::collections::HashSet;
use std::{convert::From, env, fs, ops::Add};


#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl From<&Direction> for Point {
    fn from(dir: &Direction) -> Self {
        match dir {
            Direction::Up => Point { x: 0, y: 1 },
            Direction::Down => Point { x: 0, y: -1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }
}

impl Add for &Point {
    type Output = Point;
    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Point {
  fn compute_norm(&self) -> i32 {
    self.x.abs () + self.y.abs()
  }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&String> for Direction {
    fn from(string: &String) -> Self {
        match string.as_str() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("unknown direction {:}", string),
        }
    }
}

struct WireSegment {
    direction: Direction,
    distance: usize,
}

impl WireSegment {
    fn to_vector(self, start: &Point) -> Vec<Point> {
        let mut vec = Vec::with_capacity(self.distance);
        let mut pos = *start;
        let delta = Point::from(&self.direction);
        for _ in 0..self.distance {
            pos = &pos + &delta;
            vec.push(pos);
        }
        vec
    }
}

impl From<String> for WireSegment {
    fn from(string: String) -> Self {
        let mut dir_part = string;
        let dist_part = dir_part.split_off(1);

        let dir = Direction::from(&dir_part);
        let dist = dist_part.parse::<usize>().unwrap();

        WireSegment {
            direction: dir,
            distance: dist,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let contents = fs::read_to_string(input).unwrap();

    let wires: Vec<Vec<Point>> = contents
        .trim()
        .split_whitespace()
        .map(String::from)
        .map(parse_wire)
        .collect();

    let wire1 = &wires[0];
    let wire2 = &wires[1];

    let crossings = find_duplicates(wire1, wire2);

    let mut norms: Vec<i32> = crossings.into_iter().map(|c| c.compute_norm()).collect();
    norms.sort();
    let closest = norms [0];

    println!("{}", closest)
}

fn parse_wire(string: String) -> Vec<Point> {
    let segments = string.split(",").map(String::from).map(WireSegment::from);

    let mut out = Vec::new();
    let mut start = Point { x: 0, y: 0 };

    for segment in segments {
        let mut points = segment.to_vector(&start);
        start = *points.last().unwrap();
        out.append(&mut points);
    }

    out
}

fn find_duplicates(wire1: &Vec<Point>, wire2: &Vec<Point>) -> Vec<Point> {
    let points1: HashSet<Point> = wire1.into_iter().copied().collect();
    let points2: HashSet<Point> = wire2.into_iter().copied().collect();

    let duplicates = points1.intersection(&points2);
    duplicates.copied().collect()
}
