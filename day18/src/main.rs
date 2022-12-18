use std::{fmt::Pointer, fs, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}
impl Point {
    /// Return true if the other point is touching us
    pub(crate) fn adjacent(&self, other: &&Point) -> bool {
        // dbg!((self.x - other.x).abs());
        (self.x == other.x && self.y == other.y && (self.z - other.z).abs() <= 1)
            || (self.y == other.y && self.z == other.z && (self.x - other.x).abs() <= 1)
            || (self.z == other.z && self.x == other.x && (self.y - other.y).abs() <= 1)
    }

    pub(crate) fn new(x: isize, y: isize, z: isize) -> Self {
        Point { x, y, z }
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elements = s.split(',');
        Ok(Point {
            x: elements.next().unwrap().parse()?,
            y: elements.next().unwrap().parse()?,
            z: elements.next().unwrap().parse()?,
        })
    }
}

fn main() -> anyhow::Result<()> {
    let cubes: Vec<Point> = fs::read_to_string("input")?.lines().map(|line| line.parse().unwrap()).collect();
    // let cubes = [Point::new(1, 1, 1), Point::new(2, 1, 1)];

    let mut exposed_sides = 0;

    for cube in &cubes {
        // Test every of the 6 face for being covered by another cube
        let covered = cubes.iter().filter(|other| *cube != **other && cube.adjacent(other)).count();

        exposed_sides += 6 - covered;
    }

    println!("Exposed faces: {}", exposed_sides);

    Ok(())
}
