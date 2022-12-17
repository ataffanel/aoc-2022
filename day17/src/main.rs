use std::{ops::{Add, Sub}, str::FromStr, fs, collections::HashMap};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

// impl Sub<Point> for Point {
//     type Output = Point;

//     fn sub(self, rhs: Point) -> self::Output {
//         Point { x: self.x - rhs.x, y: self.y - rhs.y }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Shape {
    #[default]
    HBar,
    Plus,
    RL,
    VBar,
    Square,
}

impl Iterator for Shape {
    type Item = Shape;

    fn next(&mut self) -> Option<Self::Item> {
        let prev = *self;
        *self = match self {
            Self::HBar => Self::Plus,
            Self::Plus => Self::RL,
            Self::RL => Self::VBar,
            Self::VBar => Self::Square,
            Self::Square => Self::HBar,
        };

        Some(prev)
    }
}

impl Shape {
    fn get_points(&self) -> Vec<Point> {
        match self {
            Shape::HBar => vec![Point::new(0 ,0), Point::new(1, 0), Point::new(2,0), Point::new(3,0)],
            Shape::Plus => vec![Point::new(0 ,1), Point::new(1,1), Point::new(2,1), Point::new(1,0), Point::new(1, 2)],
            Shape::RL => vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0), Point::new(2, 1), Point::new(2, 2)],
            Shape::VBar => vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2), Point::new(0, 3)],
            Shape::Square => vec![Point::new(0, 0), Point::new(1, 0), Point::new(1, 1), Point::new(0, 1)],
        }
    }
}

#[derive(Clone)]
struct Rock {
    shape: Shape,
    position: Point,
}
impl Rock {
    fn does_collide(&self, cave: &Cave) -> bool {
        self.shape.get_points().iter().map(|p| *p + self.position).any(|point| !matches!(cave.get_cell_at(point), Cell::Air))
    }
}

#[derive(Debug, Default, Clone, Copy)]
enum Cell {
    #[default]
    Air,
    Wall,
    Rock,
}

#[derive(Debug, Clone, Copy)]
enum WindDirection {
    Left,
    Right,
}

impl Into<Point> for WindDirection {
    fn into(self) -> Point {
        match self {
            Self::Left => Point::new(-1, 0),
            Self::Right => Point::new(1, 0),
        }
    }
}

impl From<char> for WindDirection{
    fn from(value: char) -> Self {
        match value {
            '<' => WindDirection::Left,
            '>' => WindDirection::Right,
            _ => panic!("Invalid wind!"),
        }
    }
}

struct Wind {
    directions: Vec<WindDirection>,
    current: usize,
}

impl Wind {
    fn new(directions: Vec<WindDirection>) -> Self {
        Self {
            directions,
            current: 0,
        }
    }

    fn next(&mut self) -> WindDirection {
        let dir = self.directions[self.current];
        self.current = (self.current + 1) % self.directions.len();
        dir
    }
}

#[derive(Default)]
struct Cave {
    cave: HashMap<Point, Cell>,
}

impl Cave {
    fn get_cell_at(&self, pos: Point) -> Cell {
        match pos {
            Point{x, y: _} if x < 0 => Cell::Wall,
            Point{x, y: _} if x >= 7 => Cell::Wall,
            Point{x: _, y} if y < -0 => Cell::Wall,
            _ => self.cave.get(&pos).cloned().unwrap_or_default(),
        }
    }

    fn get_top(&self) -> isize {
        self.cave.iter().filter(|(k, v)| !matches!(v, Cell::Air)).map(|(k, _)| k.y).max().unwrap_or(-1)
    }

    fn put_rock(&mut self, rock: &Rock) {
        for point in rock.shape.get_points() {
            self.cave.insert(rock.position + point, Cell::Rock);
        }
    }

    fn print (&self) {
        for y in (0..self.get_top()+3).rev() {
            for x in -1..8 {
                match self.get_cell_at(Point::new(x,y)) {
                    Cell::Wall => print!("|"),
                    Cell::Air =>  print!("."),
                    Cell::Rock =>  print!("#"),
                }
            }
            println!();
        }
        println!();
    }
}

fn main() -> anyhow::Result<()>{
    let wind_directions: Vec<WindDirection> = fs::read_to_string("input")?.chars().map(WindDirection::from).collect();
    let mut wind = Wind::new(wind_directions);
    let mut cave = Cave::default();
    let mut shape = Shape::default();
    
    for n in 0..2022 {
        let position = Point{x: 2, y: cave.get_top() + 4};
        let mut rock = Rock { position, shape: shape.next().unwrap() };

        // Make the rock fall
        loop {
            // First the wind
            let mut test_rock = rock.clone();
            let wind_direction = wind.next();
            //dbg!(&wind_direction);
            test_rock.position = test_rock.position + wind_direction.into();
            if !test_rock.does_collide(&cave) {
                rock = test_rock;
            }

            // Then the bottom movement
            let mut test_rock = rock.clone();
            test_rock.position = test_rock.position + Point::new(0, -1);
            if !test_rock.does_collide(&cave) {
                rock = test_rock;
            } else {
                // If we cannot get down, the rock is stable in the cave
                // Put it there and drop the next rock!
                cave.put_rock(&rock);
                println!("Dropped {}!", n);

                //dbg!("Drop");

                //cave.print();
                break;
            }
        }
    }

    println!("Top of the cave: {}", cave.get_top()+1);

    Ok(())
}
