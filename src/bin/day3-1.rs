use std::io::{self, Read};
use std::str::FromStr;
use std::cmp::{min, max};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let wires: Vec<Wire> = input.lines()
        .map(|w| {
            let path = w.split(',')
                .map(|m| Move::from_str(m).unwrap())
                .collect::<Vec<Move>>();

            Wire::new_from_moves(&path)
        })
        .collect();

    let w1 = &wires[0];
    let w2 = &wires[1];

    let origin = Point::new(0,0);

    let cross = w1.find_crossings(w2)
        .iter()
        .map(|p| p.manhattan_distance(&origin))
        .min()
        .unwrap();


    println!("{:?}", cross);

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LineSegment {
    p: Point,
    q: Point
}

impl LineSegment {
    fn new(p: Point, q: Point) -> Self {
        LineSegment { p, q }
    }

    fn line_segments_from_points(points: Vec<Point>) -> Vec<LineSegment> {
        let mut segments = Vec::new();
        for (a, b) in points.iter().zip(points.iter().skip(1)) {
            segments.push(LineSegment::new(*a, *b));
        }
        segments
    }

    fn orientation(&self) -> Orientation {
        if self.p.x == self.q.x { Orientation::Vertical }
        else { Orientation::Horizontal }
    }

    fn is_point_in_x_range(&self, p: Point) -> bool {
        p.x >= min(self.p.x, self.q.x) && p.x <= max(self.p.x, self.q.x)
    }

    fn is_point_in_y_range(&self, p: Point) -> bool {
        p.y >= min(self.p.y, self.q.y) && p.y <= max(self.p.y, self.q.y)
    }


    fn intersects(&self, other: &LineSegment) -> Option<Point> {
        if self.orientation() == other.orientation() { None }
        else if self.orientation() == Orientation::Horizontal
            && self.is_point_in_x_range(other.p)
            && other.is_point_in_y_range(self.p) {
            Some(Point::new(other.p.x, self.p.y))
        } else if self.orientation() == Orientation::Vertical
            && self.is_point_in_y_range(other.p)
            && other.is_point_in_x_range(self.p) {
            Some(Point::new(self.p.x, other.p.y))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Orientation {
    Horizontal,
    Vertical
}

#[derive(Clone, Debug)]
struct Wire {
    path: Vec<LineSegment>,
}

impl Wire {
    fn new(path: Vec<LineSegment>) -> Self {
        Wire { path }
    }

    fn new_from_moves(moves: &[Move]) -> Self {
        let mut path = Vec::new();
        let mut point = Point { x: 0, y: 0 };

        for m in moves {
            match m {
                Move::Up(v) => point.y += *v,
                Move::Down(v) => point.y -= *v,
                Move::Left(v) => point.x -= *v,
                Move::Right(v) => point.x += *v,
            }
            path.push(point);
        }

        Wire::new(LineSegment::line_segments_from_points(path))
    }

    fn find_crossings(&self, other: &Wire) -> Vec<Point> {
        let mut intersections = Vec::new();
        for segment1 in &self.path {
            for segment2 in &other.path {
                match segment1.intersects(segment2) {
                    Some(i) => intersections.push(i),
                    None => {}
                }
            }
        }
        intersections
    }
}

enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Move, ()> {
        if s.len() < 2 {
            return Err(())
        }

        let letter = s.chars().nth(0).unwrap();
        let number: i32 = s[1usize..].parse::<i32>().unwrap();

        match letter {
            'U' => Ok(Move::Up(number)),
            'D' => Ok(Move::Down(number)),
            'L' => Ok(Move::Left(number)),
            'R' => Ok(Move::Right(number)),
            _ => Err(())
        }
    }
}
