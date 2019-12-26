use std::collections::{HashMap, HashSet};
use std::{convert::From, env, fs, ops::Add};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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
    fn compute_norm(&self, wire1: &HashMap<Point, i32>, wire2: &HashMap<Point, i32>) -> i32 {
        let dist1 = wire1.get(self).unwrap();
        let dist2 = wire2.get(self).unwrap();

        dist1 + dist2
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
    fn to_distances(self, mut pos: Point, mut dist: i32) -> (HashMap<Point, i32>, Point, i32) {
        let mut map = HashMap::new();
        let delta = Point::from(&self.direction);
        for _ in 0..self.distance {
            pos = &pos + &delta;
            dist += 1;
            if !map.contains_key(&pos) {
                map.insert(pos, dist);
            }
        }

        (map, pos, dist)
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

    let wires: Vec<HashMap<Point, i32>> = contents
        .trim()
        .split_whitespace()
        .map(String::from)
        .map(parse_wire)
        .collect();

    let wire1 = &wires[0];
    let wire2 = &wires[1];

    let crossings = find_duplicates(wire1, wire2);

    let mut norms: Vec<i32> = crossings
        .into_iter()
        .map(|c| c.compute_norm(wire1, wire2))
        .collect();
    norms.sort();
    let closest = norms[0];

    println!("{}", closest)
}

fn parse_wire(string: String) -> HashMap<Point, i32> {
    let segments = string.split(",").map(String::from).map(WireSegment::from);

    let mut maps = Vec::new();
    let mut pos = Point { x: 0, y: 0 };
    let mut dist = 0;

    for segment in segments {
        let (points, new_pos, new_dist) = segment.to_distances(pos, dist);
        pos = new_pos;
        dist = new_dist;
        maps.push(points);
    }
    maps.reverse();

    let mut out = HashMap::new();
    for map in maps {
        out.extend(map);
    }

    out
}

fn find_duplicates(wire1: &HashMap<Point, i32>, wire2: &HashMap<Point, i32>) -> Vec<Point> {
    let points1: HashSet<Point> = wire1.keys().into_iter().copied().collect();
    let points2: HashSet<Point> = wire2.keys().into_iter().copied().collect();

    let duplicates = points1.intersection(&points2);
    duplicates.copied().collect()
}
