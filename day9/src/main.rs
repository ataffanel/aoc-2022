use std::{
    collections::HashSet,
    fs::{self, File},
    io::{BufRead, BufReader},
    ops::Index,
    str::FromStr,
};

use anyhow::{Error, Result};

#[derive(Debug, Hash, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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
        (self.x - other.x).abs() < 2 && (self.y - other.y).abs() < 2
    }
}

struct Bridge<const N: usize> {
    head: Point,
    tails: [Point; N],
}

impl<const N: usize> Bridge<N> {
    fn new() -> Self {
        Bridge {
            head: Point { x: 0, y: 0 },
            tails: [Point { x: 0, y: 0 }; N],
        }
    }

    fn move_head(&mut self, direction: Direction) {
        self.head.r#move(direction);

        for t in 0..N {
            match t {
                0 => self.tails[0].chase(&self.head),
                _ => {
                    let tail_to_chase = self.tails[t - 1];
                    self.tails[t].chase(&tail_to_chase);
                }
            }
        }
    }

    fn display(&self, visited: &[Point]) {
        for y in (-20..20).rev() {
            for x in -20..20 {
                match Point::new(x, y) {
                    p if p == self.head => print!("H"),
                    p if self.tails.contains(&p) => {
                        print!("{}", self.tails.iter().position(|t| *t == p).unwrap() + 1)
                    }
                    p if visited.contains(&p) => print!("#"),
                    Point { x: 0, y: 0 } => print!("s"),
                    _ => print!("."),
                }
            }
            println!("");
        }
    }

    fn tail(&self) -> Point {
        *self.tails.last().unwrap()
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
        match s {
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
        (
            Direction::from_str(elements.next().unwrap()).unwrap(),
            elements.next().unwrap().parse().unwrap(),
        )
    });

    let mut bridge = Bridge::<1>::new();
    let mut visited = Vec::new();

    for (direction, n) in moves {
        for _ in 0..n {
            bridge.move_head(direction);
            if !visited.contains(&bridge.tail()) {
                visited.push(bridge.tail());
            }

            // println!("{:?} {}", direction, n);
            // bridge.display(&visited);
            // println!("");
        }
    }

    println!("Visited locations: {}", visited.len());

    // Part 2

    let moves = BufReader::new(File::open("input")?).lines().map(|l| {
        let l = l.unwrap();
        let mut elements = l.split(" ");
        (
            Direction::from_str(elements.next().unwrap()).unwrap(),
            elements.next().unwrap().parse().unwrap(),
        )
    });

    let mut bridge = Bridge::<9>::new();
    let mut visited = Vec::new();

    for (direction, n) in moves {
        for _ in 0..n {
            bridge.move_head(direction);
            if !visited.contains(&bridge.tail()) {
                visited.push(bridge.tail());
            }

            // println!("{:?} {}", direction, n);
            // bridge.display(&visited);
            // println!("");
        }
    }

    bridge.display(&visited);
    println!("");

    println!("Visited locations: {}", visited.len());

    // dbg!(moves.collect::<Vec<_>>());

    Ok(())
}
