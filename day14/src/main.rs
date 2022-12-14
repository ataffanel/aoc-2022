use std::{collections::HashMap, fs, str::FromStr};

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl FromStr for Position {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elements = s.split(',');
        Ok(Position {
            x: elements.next().unwrap().parse()?,
            y: elements.next().unwrap().parse()?,
        })
    }
}

impl Position {
    fn down(self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn down_left(self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn down_right(self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

#[derive(Debug, Default)]
enum Cell {
    #[default]
    Air,
    Rock,
    Sand,
}

#[derive(Debug)]
struct Cave {
    cave: HashMap<Position, Cell>,
    bottom: usize,
}

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cave = HashMap::new();
        let mut bottom = 0;

        for line in s.lines() {
            let mut corners = line.split(" -> ");
            let mut prev: Position = corners.next().unwrap().parse()?;

            while let Some(next) = corners.next().map(|c| c.parse::<Position>().unwrap()) {
                if prev.x == next.x {
                    let x = prev.x;
                    let start_y = next.y.min(prev.y);
                    let stop_y = next.y.max(prev.y);
                    for y in start_y..=stop_y {
                        cave.insert(Position { x, y }, Cell::Rock);
                    }
                }
                if prev.y == next.y {
                    let y = prev.y;
                    let start_x = next.x.min(prev.x);
                    let stop_x = next.x.max(prev.x);
                    for x in start_x..=stop_x {
                        cave.insert(Position { x, y }, Cell::Rock);
                    }
                }

                bottom = bottom.max(next.y);

                prev = next;
            }
        }

        Ok(Cave { cave, bottom })
    }
}

impl Cave {
    fn sand_step(&mut self) -> bool {
        let mut sand = Position { x: 500, y: 0 };

        loop {
            if let Cell::Air = self.cave.get(&sand.down()).unwrap_or(&Cell::Air) {
                sand = sand.down();
            } else if let Cell::Air = self.cave.get(&sand.down_left()).unwrap_or(&Cell::Air) {
                sand = sand.down_left();
            } else if let Cell::Air = self.cave.get(&sand.down_right()).unwrap_or(&Cell::Air) {
                sand = sand.down_right();
            } else {
                self.cave.insert(sand, Cell::Sand);
                break;
            }

            // dbg!(sand);

            if sand.y > self.bottom {
                self.cave.insert(sand, Cell::Sand);
                break;
            }
        }

        sand == Position { x: 500, y: 0 }
    }
}

fn main() -> anyhow::Result<()> {
    let mut cave: Cave = fs::read_to_string("input")?.parse()?;

    let mut n_sand = 0;

    // dbg!(&cave);

    while !cave.sand_step() {
        n_sand += 1;
    }

    println!("Number of sand drop to close the source: {}", n_sand + 1);

    Ok(())
}
