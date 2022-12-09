use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    str::FromStr, collections::HashSet,
};

use anyhow::{Error, Result};

#[derive(Debug, Hash, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x, y
        }
    }

    fn chase(&mut self, other: &Point) {

        while !self.close_from(other) {
            if self.x < other.x {
                self.x += 1;
            } else if self.x > other.x {
                self.x -= 1;
            }
    
            if self.y < other.y {
                self.y += 1;
            } else if self.y > other.y {
                self.y -= 1;
            }
        }
    }

    fn r#move(&mut self, r#move: Direction) {
        match r#move {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn close_from(&self, other: &Point) -> bool {
        (self.x-other.x).abs() < 2 && (self.y-other.y).abs() < 2
    }
}

struct Bridge {
    head: Point,
    tail: Point,
}

impl Bridge {
    fn new() -> Self {
        Bridge {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
        }
    }
}

impl Bridge {
    fn move_head(&mut self, direction: Direction) {
        self.head.r#move(direction);
        self.tail.chase(&self.head);
    }

    fn display(&self, visited: &[Point]) {
        for y in (0..7).rev() {
            for x in 0..7 {
                match Point::new(x, y) {
                    p if p == self.head => print!("H"),
                    p if p == self.tail => print!("T"),
                    p if visited.contains(&p) => print!("#"),
                    Point{x: 0, y: 0} => print!("s"),
                    _ => print!("."),
                }
            }
            println!("");
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(Error::msg("Bad input data")),
        }
    }
}

fn main() -> Result<()> {
    let moves = BufReader::new(File::open("input")?).lines().map(|l| {
        let l = l.unwrap();
        let mut elements = l.split(" ");
        (Direction::from_str(elements.next().unwrap()).unwrap(), elements.next().unwrap().parse().unwrap())
    });

    let mut bridge = Bridge::new();
    let mut visited = Vec::new();

    for (direction, n) in moves {
        for _ in 0..n{
            bridge.move_head(direction);
            if !visited.contains(&bridge.tail) {
                visited.push(bridge.tail);
            }

            // dbg!(m.0, bridge.head, bridge.tail);
            // println!("{:?} {}", direction, n);
            // bridge.display(&visited);
            // println!("");
        }
    }

    println!("Visited locations: {}", visited.len());

    // dbg!(moves.collect::<Vec<_>>());

    Ok(())
}
