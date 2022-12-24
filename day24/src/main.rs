use std::{collections::BTreeSet, str::FromStr, fs, ops::Add};

use anyhow::{Result, Error};

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
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

#[derive(Debug)]
struct Maze {
    wind: Vec<(Point, Direction)>,
    width: isize,
    height: isize,
}

impl FromStr for Maze {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        
        // First line is a header, allows us to get the width
        let width = lines.next().unwrap().len() - 2;
        
        // Then comes the wind
        let mut wind = Vec::new();
        let mut height = 0;
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().skip(1).enumerate() {
                let position = Point::new(x as isize, y as isize);
                match c {
                    '^' => wind.push((position, Direction::Up)),
                    '>' => wind.push((position, Direction::Right)),
                    'v' => wind.push((position, Direction::Down)),
                    '<' => wind.push((position, Direction::Left)),
                    _ => (),
                }
            }
            height = y;
        }
        
        Ok(Maze {
            wind,
            width: width as isize,
            height: height as isize,
        })
    }
}

impl Maze {
    fn move_wind(&mut self) {
        for (position, direction) in self.wind.iter_mut() {
            match direction {
                Direction::Up => {
                    let mut new_y = position.y - 1;
                    if new_y < 0 {
                        new_y = self.height-1;
                    }
                    *position = Point::new(position.x, new_y);
                },
                Direction::Right => {
                    let mut new_x = position.x + 1;
                    if new_x > self.width-1 {
                        new_x = 0;
                    }
                    *position = Point::new(new_x, position.y);
                },
                Direction::Down => {
                    let mut new_y = position.y + 1;
                    if new_y > self.height-1 {
                        new_y = 0;
                    }
                    *position = Point::new(position.x, new_y);
                },
                Direction::Left => {
                    let mut new_x = position.x - 1;
                    if new_x < 0 {
                        new_x = self.width - 1;
                    }
                    *position = Point::new(new_x, position.y);
                },
            }
        }
    }
    
    fn solve_min_distance(&mut self) -> usize {
        
        let mut round = 0;
        
        // We start at the start!
        let mut state = BTreeSet::from_iter(vec![Point::new(0,0)].into_iter());
        
        loop {
            self.move_wind();
            let wind_position: BTreeSet<Point> = self.wind.iter().map(|(position, _)| *position).collect();
            
            state = state.iter().map(|position| {
                let mut new_states = Vec::new();
                
                for offset in [
                    Point::new(0,0), Point::new(-1, 0), Point::new(1, 0),
                    Point::new(0, 1), Point::new(0, -1)
                ] {
                    let to_try = *position + offset;
                    
                    if !wind_position.contains(&to_try) {
                        if to_try.x >= 0 && to_try.y < self.width && to_try.y >= 0 && to_try.y < self.height {
                            new_states.push(to_try);
                        }
                    }
                    
                    if to_try == Point::new(self.width-1, self.height) {
                        new_states.push(to_try);
                    }
                }
                                
                new_states
            }).flatten().collect();
            
            if state.iter().any(|p| *p == Point::new(self.width-1, self.height)) {
                break;
            }
            
            round += 1;
        }
        
        round+1
    }
    
    fn print(&self) {
        for y in -1..self.height+1 {
            print!("#");
            for x in 0..self.width {
                if y < 0 && x > 0 {
                    print!("#");
                } else if y >= self.height && x < self.width-1 {
                    print!("#");
                }else {
                    match self.wind.iter().find(|(pos, _)| *pos == Point::new(x, y)) {
                        Some((_, Direction::Up)) => print!("^"),
                        Some((_, Direction::Right)) => print!(">"),
                        Some((_, Direction::Down)) => print!("v"),
                        Some((_, Direction::Left)) => print!("<"),
                        None => print!("."),
                    }
                }
                
            }
            println!("#")
        }
    }
}

fn main() -> Result<()> {
    let mut maze: Maze = fs::read_to_string("input")?.parse()?;
    
    println!("Min distance to goal: {}", maze.solve_min_distance());
    
    
    
    Ok(())
}
