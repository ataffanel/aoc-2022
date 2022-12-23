use std::{
    collections::{HashMap, VecDeque},
    fs,
    ops::Add,
    str::FromStr,
};

use anyhow::{Error, Result};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

enum Tile {
    Elf,
}

struct Map {
    map: HashMap<Point, Tile>,
    checks: VecDeque<Vec<Point>>,
}

impl Map {
    // Returns Elf's next move
    fn next_move(&self, elf: Point) -> Point {
        // Check if there is any neighbor
        let mut has_neighbor = false;
        for x in -1..=1 {
            for y in -1..=1 {
                if !(x == 0 && y == 0) {
                    let to_test = elf + Point::new(x, y);
                    // dbg!(Point::new(x, y), to_test);
                    has_neighbor |= self.map.contains_key(&to_test);
                    // dbg!(self.map.contains_key(&to_test));
                }
            }
        }

        if !has_neighbor {
            // println!("Elf {:?} is alone, not moving!", elf);
            return elf;
        } else {
            // println!("Trying to move {:?}", elf);
        }

        for checks in &self.checks {
            let can_move = !checks.iter().any(|offset| {
                let to_check = elf + *offset;
                self.map.contains_key(&to_check)
            });

            if can_move {
                return elf + checks[0];
            }
        }

        elf
    }

    fn move_elves(&mut self) -> bool {
        let mut new_map = HashMap::new();
        let mut any_moved = false;
        let mut collisions: HashMap<Point, usize> = HashMap::new();

        // Pre-calculate collisions:
        for pos in self.map.keys() {
            let new_pos = self.next_move(*pos);

            let n: usize = collisions.get(&new_pos).cloned().unwrap_or_default();
            collisions.insert(new_pos, n + 1);
        }

        dbg!(&self.checks[0]);

        for pos in self.map.keys() {
            let new_pos = self.next_move(*pos);

            let should_move = collisions[&new_pos] == 1;

            // let should_move = true;
            any_moved |= new_pos != *pos;

            // dbg!(should_move);
            if should_move {
                // println!("Moving {:?} to {:?}", pos, new_pos);
                new_map.insert(new_pos, Tile::Elf);
            } else {
                // println!("Not moving {:?}", pos);
                new_map.insert(*pos, Tile::Elf);
            }
        }

        self.map = new_map;

        // Rotate the checks
        let checks = self.checks.pop_front().unwrap();
        self.checks.push_back(checks);

        any_moved
    }

    fn count_space(&self) -> usize {
        let min_x = self.map.keys().map(|p| p.x).min().unwrap();
        let max_x = self.map.keys().map(|p| p.x).max().unwrap();
        let min_y = self.map.keys().map(|p| p.y).min().unwrap();
        let max_y = self.map.keys().map(|p| p.y).max().unwrap();

        let mut count = 0;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if !self.map.contains_key(&Point::new(x, y)) {
                    count += 1;
                }
            }
        }

        count
    }

    fn print(&self) {
        let min_x = self.map.keys().map(|p| p.x).min().unwrap();
        let max_x = self.map.keys().map(|p| p.x).max().unwrap();
        let min_y = self.map.keys().map(|p| p.y).min().unwrap();
        let max_y = self.map.keys().map(|p| p.y).max().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.map.contains_key(&Point::new(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => (),
                    '#' => {
                        map.insert(Point::new(x as isize, y as isize), Tile::Elf);
                    }
                    _ => return Err(Error::msg("Bad input format")),
                };
            }
        }

        // Create list of checks
        let checks = VecDeque::from_iter([
            vec![Point::new(0, -1), Point::new(1, -1), Point::new(-1, -1)],
            vec![Point::new(0, 1), Point::new(1, 1), Point::new(-1, 1)],
            vec![Point::new(-1, 0), Point::new(-1, 1), Point::new(-1, -1)],
            vec![Point::new(1, 0), Point::new(1, 1), Point::new(1, -1)],
        ]);

        Ok(Map { map, checks })
    }
}

fn main() -> Result<()> {
    let mut map: Map = fs::read_to_string("input")?.parse()?;

    map.print();

    let mut round = 1;
    loop {
        println!("Round {} --------------", round);
        let any_moved = map.move_elves();
        map.print();

        if !any_moved {
            break;
        }

        if round == 10 {
            println!("Space left after 10 round (part 1): {}", map.count_space());
        }
        round += 1;
    }

    println!("No one moved after {} rounds (Part 2)", round);

    Ok(())
}
