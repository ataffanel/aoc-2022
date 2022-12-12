use std::{fs, str::FromStr, fmt::{self, Display}};

use colorful::Colorful;

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Terrain {
    Free(u8),
    Visited(u8),
    Start,
    End,
}

impl Into<Terrain> for char {
    fn into(self) -> Terrain {
        match self {
            'S' => Terrain::Start,
            'E' => Terrain::End,
            'a'..='z' => Terrain::Free(self as u8 - 'a' as u8),
            _ => panic!("Wrong input!"),
        }
    }
}

impl Terrain {
    fn height(self) -> u8 {
        match self {
            Terrain::End => 25,
            Terrain::Start => 0,
            Terrain::Free(n) => n,
            Terrain::Visited(n) => n,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn left(&self) -> Self {
        Position { x: self.x-1, y: self.y }
    }

    fn right(&self) -> Self {
        Position {x: self.x+1, y: self.y}
    }

    fn up(&self) -> Self {
        Position { x: self.x, y: self.y-1 }
    }

    fn down(&self) -> Self {
        Position {x: self.x, y: self.y+1}
    }

    fn move_to(&mut self, direction: Direction) {
        match direction {
            Direction::Up => *self = self.up(),
            Direction::Down => *self = self.down(),
            Direction::Left => *self = self.left(),
            Direction::Right => *self = self.right(),
        }
    }

    fn set_position(&mut self, x: isize, y: isize) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Debug)]
struct Maze {
    field: Vec<Vec<Terrain>>,
    distances: Vec<Vec<usize>>,
    to_visit: VecDeque<Position>,
    end: Position,
    cursor: Position,
    reverse: bool,
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Position::default();
        let mut end = Position::default();

        let field: Vec<Vec<Terrain>> = s
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();

        for (x, line) in field.iter().enumerate() {
            for (y, terrain) in line.iter().enumerate() {
                match terrain {
                    Terrain::Start => start.set_position(x as isize, y as isize),
                    Terrain::End => end.set_position(x as isize, y as isize),
                    _ => (),
                }
            }
        }

        let height = field.len();
        let width = field[0].len();

        let mut to_visit = VecDeque::new();
        to_visit.push_back(start);

        Ok(Self {
            field,
            distances: vec![vec![0; width]; height],
            to_visit,
            end,
            cursor: start,
            reverse: false,
        })
    }
}

impl Maze {
    fn get_tile(&self, p: Position) -> Option<Terrain> {
        if p.x < 0 || p.x as usize >= self.field.len() || p.y < 0 || p.y as usize >= self.field[0].len() {
            None
        } else {
            Some(self.field[p.x as usize][p.y as usize])
        }
    }

    fn get_neighbor_tile(&self, direction: Direction) -> Option<Terrain>{
        match direction {
            Direction::Up => self.get_tile(self.cursor.up()),
            Direction::Down => self.get_tile(self.cursor.down()),
            Direction::Left => self.get_tile(self.cursor.left()),
            Direction::Right => self.get_tile(self.cursor.right()),
        }
    }

    fn set_tile(&mut self, p: Position, tile: Terrain) {
        self.field[p.x as usize][p.y as usize] = tile;
    }

    fn set_neighbor_tile(&mut self, direction: Direction, tile: Terrain) {
        match direction {
            Direction::Up => self.set_tile(self.cursor.up(), tile),
            Direction::Down => self.set_tile(self.cursor.down(), tile),
            Direction::Left => self.set_tile(self.cursor.left(), tile),
            Direction::Right => self.set_tile(self.cursor.right(), tile),
        }
    }

    fn get_distance(&self, p: Position) -> Option<usize> {
        if p.x < 0 || p.x as usize >= self.field.len() || p.y < 0 || p.y as usize >= self.field[0].len() {
            None
        } else {
            Some(self.distances[p.x as usize][p.y as usize])
        }
    }

    fn set_distance(&mut self, p: Position, distance: usize) {
        self.distances[p.x as usize][p.y as usize] = distance;
    }

    fn set_neighbor_distance(&mut self, direction: Direction, distance: usize) {
        match direction {
            Direction::Up => self.set_distance(self.cursor.up(), distance),
            Direction::Down => self.set_distance(self.cursor.down(), distance),
            Direction::Left => self.set_distance(self.cursor.left(), distance),
            Direction::Right => self.set_distance(self.cursor.right(), distance),
        }
    }

    // Walk one step of the BFS path finding algorithm
    // Returns true when we have reached the end!
    fn step(&mut self) -> bool {
        if let Some(position) = self.to_visit.pop_front() {

            let current_distance = self.get_distance(position).unwrap();

             for direction in [Direction::Up, Direction::Left, Direction::Right, Direction::Down] {
                self.cursor = position;
                if let Some(tile) = self.get_neighbor_tile(direction) {
                    if !self.reverse {
                        if matches!(tile, Terrain::Free(_) | Terrain::End) && tile.height() < self.get_tile(self.cursor).unwrap().height() + 2 {
                            self.set_neighbor_distance(direction, current_distance + 1);
                            let mut neighbor = position.clone();
                            neighbor.move_to(direction);
                            self.to_visit.push_back(neighbor);
                            self.set_neighbor_tile(direction, Terrain::Visited(tile.height()));
                        }
                    } else {
                        if matches!(tile, Terrain::Free(_) | Terrain::Start) && tile.height() + 2 > self.get_tile(self.cursor).unwrap().height()  {
                            self.set_neighbor_distance(direction, current_distance + 1);
                            let mut neighbor = position.clone();
                            neighbor.move_to(direction);
                            self.to_visit.push_back(neighbor);
                            self.set_neighbor_tile(direction, Terrain::Visited(tile.height()));
                        }
                    }
                    
                }
             }

            false
        } else {
            true
        }
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        for line in &self.field {
            let line = line.iter().map(|tile| {
                match tile {
                    Terrain::Visited(n) => {
                        let c = (n + 'a' as u8) as char;
                        format!("{}", c.to_string().red())
                    }
                    Terrain::Free(n) => {
                        let c = (n + 'a' as u8) as char;
                        format!("{}", c.to_string().hsl(0.5f32, 1.0f32, *n as f32 / 25.0f32))
                    }
                    Terrain::Start => "S".to_owned(),
                    Terrain::End => "E".to_owned(),
                }
            }).fold(String::new(), |acc, s| {
                let mut res = acc.clone();
                res.push_str(&s);
                res
            });
            f.write_str(&line)?;
            f.write_str(&"\n".to_owned())?;
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let mut maze = fs::read_to_string("input")?.parse::<Maze>()?;

    println!("{}", &maze);
    println!("------\n");

    while !maze.step() {}

    println!("{}", &maze);
    println!("Distance to the end: {:?}", maze.get_distance(maze.end).unwrap());
    println!("------\n");

    // Running the algorithm the other way arround ...
    let mut maze = fs::read_to_string("input")?.parse::<Maze>()?;
    maze.to_visit.pop_back();
    maze.to_visit.push_back(maze.end);
    maze.reverse = true;

    while !maze.step() {}

    println!("{}", &maze);

    // Find the 'a' (0) the closest to the top
    let mut minimum_distance = 1000;

    for x in 0..maze.distances.len() {
        for y in 0..maze.distances[0].len() {
            let p = Position{x: x as isize, y: y as isize};
            if let Some(Terrain::Visited(0)) = maze.get_tile(p) {
                if maze.get_distance(p).unwrap() < minimum_distance {
                    minimum_distance = maze.get_distance(p).unwrap();
                }
            }
        }
    }

    println!("Minimum distance to 'a': {}", minimum_distance);

    Ok(())
}
