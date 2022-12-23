use std::{collections::HashMap, fs, hash::Hash, str::FromStr};

use anyhow::{Error, Result};

#[derive(Debug, Clone, Copy, Default)]
enum Direction {
    Up = 3,
    Left = 2,
    Down = 1,
    #[default]
    Right = 0,
}

impl Direction {
    fn turn(&mut self, turn: Turn) {
        match turn {
            Turn::R => match self {
                Direction::Up => *self = Direction::Right,
                Direction::Left => *self = Direction::Up,
                Direction::Down => *self = Direction::Left,
                Direction::Right => *self = Direction::Down,
            },
            Turn::L => match self {
                Direction::Up => *self = Direction::Left,
                Direction::Left => *self = Direction::Down,
                Direction::Down => *self = Direction::Right,
                Direction::Right => *self = Direction::Up,
            },
        }
    }
}

#[derive(Debug)]
enum Turn {
    R,
    L,
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

#[derive(Debug, Default, Clone)]
struct Cursor {
    position: Point,
    direction: Direction,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Open,
    Wall,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Open),
            '#' => Ok(Tile::Wall),
            _ => Err(Error::msg("Tile character not handled")),
        }
    }
}

#[derive(Debug)]
struct Board {
    map: HashMap<Point, Tile>,
    visited: HashMap<Point, Direction>,
    cursor: Cursor,
}

impl FromStr for Board {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                if let Ok(tile) = tile.try_into() {
                    map.insert(Point::new(x as isize, y as isize), tile);
                }
            }
        }

        let start_x = map
            .iter()
            .filter(|(p, _)| p.y == 0)
            .map(|(p, _)| p.x)
            .min()
            .unwrap();
        let cursor = Cursor {
            position: Point::new(start_x, 0),
            ..Default::default()
        };

        Ok(Board {
            map,
            visited: HashMap::new(),
            cursor,
        })
    }
}

impl Board {
    pub(crate) fn move_curor(&mut self, step: Path) -> Result<()> {
        match step {
            Path::Turn(turn) => self.cursor.direction.turn(turn),
            Path::Forward(distance) => {
                for _ in 0..distance {
                    if let Tile::Open = self.get_next_tile() {
                        self.advance_position();
                    }
                    self.visited
                        .insert(self.cursor.position, self.cursor.direction);
                }
            }
        }

        Ok(())
    }

    fn next_tile_position(&self) -> Point {
        match self.cursor.direction {
            Direction::Up => {
                let new_pos = Point {
                    x: self.cursor.position.x,
                    y: self.cursor.position.y - 1,
                };
                if let Some(_) = self.map.get(&new_pos) {
                    new_pos
                } else {
                    let new_y = self
                        .map
                        .iter()
                        .filter(|(p, _)| p.x == self.cursor.position.x)
                        .map(|(p, _)| p.y)
                        .max()
                        .unwrap();
                    Point {
                        x: self.cursor.position.x,
                        y: new_y,
                    }
                }
            }
            Direction::Left => {
                let new_pos = Point {
                    x: self.cursor.position.x - 1,
                    y: self.cursor.position.y,
                };
                if let Some(_) = self.map.get(&new_pos) {
                    new_pos
                } else {
                    let new_x = self
                        .map
                        .iter()
                        .filter(|(p, _)| p.y == self.cursor.position.y)
                        .map(|(p, _)| p.x)
                        .max()
                        .unwrap();
                    Point {
                        x: new_x,
                        y: self.cursor.position.y,
                    }
                }
            }
            Direction::Down => {
                let new_pos = Point {
                    x: self.cursor.position.x,
                    y: self.cursor.position.y + 1,
                };
                if let Some(_) = self.map.get(&new_pos) {
                    new_pos
                } else {
                    let new_y = self
                        .map
                        .iter()
                        .filter(|(p, _)| p.x == self.cursor.position.x)
                        .map(|(p, _)| p.y)
                        .min()
                        .unwrap();
                    Point {
                        x: self.cursor.position.x,
                        y: new_y,
                    }
                }
            }
            Direction::Right => {
                let new_pos = Point {
                    x: self.cursor.position.x + 1,
                    y: self.cursor.position.y,
                };
                if let Some(_) = self.map.get(&new_pos) {
                    new_pos
                } else {
                    let new_x = self
                        .map
                        .iter()
                        .filter(|(p, _)| p.y == self.cursor.position.y)
                        .map(|(p, _)| p.x)
                        .min()
                        .unwrap();
                    Point {
                        x: new_x,
                        y: self.cursor.position.y,
                    }
                }
            }
        }
    }

    fn get_next_tile(&self) -> Tile {
        self.map[&self.next_tile_position()]
    }

    fn advance_position(&mut self) {
        self.cursor.position = self.next_tile_position();
    }

    fn print_board(&self) {
        let mut y = 0;
        loop {
            let line: HashMap<_, _> = self.map.iter().filter(|(p, _)| p.y == y).collect();
            if line.is_empty() {
                break;
            }

            let max_x = line.iter().map(|(p, _)| p.x).max().unwrap();

            for x in 0..=max_x {
                match self.visited.get(&Point::new(x, y)) {
                    Some(_) => print!("o"),
                    None => match self.map.get(&Point::new(x, y)) {
                        Some(Tile::Open) => print!("."),
                        Some(Tile::Wall) => print!("#"),
                        None => print!(" "),
                    },
                }
            }
            println!();
            y += 1;
        }
    }
}

#[derive(Debug)]
enum Path {
    Turn(Turn),
    Forward(usize),
}

fn parse_path(s: &str) -> Result<Vec<Path>> {
    let mut path = Vec::new();
    let mut num = String::new();

    for c in s.chars() {
        match c {
            'R' => {
                if !num.is_empty() {
                    path.push(Path::Forward(num.parse()?));
                    num = String::new();
                }
                path.push(Path::Turn(Turn::R));
            }
            'L' => {
                if !num.is_empty() {
                    path.push(Path::Forward(num.parse()?));
                    num = String::new();
                }
                path.push(Path::Turn(Turn::L));
            }
            _ => num.push(c),
        }
    }

    if !num.is_empty() {
        path.push(Path::Forward(num.parse()?));
    }

    Ok(path)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input")?;
    let mut input = input.split("\n\n");

    let mut board: Board = input.next().unwrap().parse()?;
    let path = parse_path(input.next().unwrap())?;

    board.print_board();

    for step in path {
        // dbg!(&step, &board.cursor.direction, &board.cursor.position);
        board.move_curor(step)?;

        // println!();
        // board.print_board();
    }

    dbg!(board.cursor.position);

    let password = 1000 * (board.cursor.position.y + 1)
        + 4 * (board.cursor.position.x + 1)
        + (board.cursor.direction as isize);

    println!("Password is: {}", password);

    Ok(())
}
