use std::{
    collections::{HashMap, VecDeque},
    fs,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
enum Cell {
    #[default]
    Air,
    Steam,
    Rock,
}

fn print_map(map: &HashMap<Point, Cell>) {
    let max_x = map.iter().map(|p| p.0.x).max().unwrap() as isize;
    let max_y = map.iter().map(|p| p.0.y).max().unwrap() as isize;
    let max_z = map.iter().map(|p| p.0.z).max().unwrap() as isize;

    for z in 0..max_z+1 {
        for y in 0..max_y+1 {
            for x in 0..max_x+1 {
                match map.get(&Point{x,y,z}).cloned().unwrap_or_default() {
                    Cell::Air => print!("."),
                    Cell::Steam => print!("~"),
                    Cell::Rock => print!("#"),
                }
            }
            println!();
        }

        println!("----------");
    }
}

fn main() -> anyhow::Result<()> {
    let cubes: Vec<Point> = fs::read_to_string("input")?
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    // let cubes = [Point::new(1, 1, 1), Point::new(2, 1, 1)];

    let mut exposed_sides = 0;

    for cube in &cubes {
        // Test every of the 6 face for being covered by another cube
        let covered = cubes
            .iter()
            .filter(|other| *cube != **other && cube.adjacent(other))
            .count();

        exposed_sides += 6 - covered;
    }

    println!("Exposed faces: {}", exposed_sides);

    // Part2: To isolate pockets, we create a 3D MAP then fill the outdide with steam
    // The new result is the number of adjacent rock to steam
    let mut map = HashMap::new();
    let max_x = cubes.iter().map(|p| p.x).max().unwrap() as isize;
    let max_y = cubes.iter().map(|p| p.y).max().unwrap() as isize;
    let max_z = cubes.iter().map(|p| p.z).max().unwrap() as isize;

    dbg!(max_x, max_y, max_z);

    for cube in cubes {
        map.insert(cube, Cell::Rock);
    }

    // Fill up the vapor
    let mut to_visit = VecDeque::new();
    to_visit.push_front(Point::new(0, 0, 0));

    while let Some(current) = to_visit.pop_back() {
        // println!("=========\nCurrent: {:?}", current);
        

        // dbg!(&to_visit);
        // visit all neighbor that are air
        // for x in -1..=1 {
        //     for y in -1..=1 {
        //         for z in -1..=1 {
        for (x, y, z) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0,  1),
        ] {
                    let neighbor = Point {
                        x: current.x + x,
                        y: current.y + y,
                        z: current.z + z,
                    };
                    // dbg!(neighbor);
                    if neighbor.x < -1
                        || neighbor.x > max_x + 1
                        || neighbor.y < 0
                        || neighbor.y > max_y + 1
                        || neighbor.z < 0
                        || neighbor.z > max_z + 1
                    {
                        // println!("Dropping {:?} because out of bound", neighbor);
                        continue;
                    }
                    // dbg!(neighbor);

                    if let Cell::Air = map.get(&neighbor).cloned().unwrap_or_default() {
                        // dbg!(neighbor);
                        map.insert(neighbor, Cell::Steam);
                        to_visit.push_front(neighbor);
                //     }
                // }
            }
        }

        // print_map(&map);
        // println!("{:?}", to_visit);
    }

    print_map(&map);

    let n_steam = map
        .iter()
        .filter(|(_, cell)| matches!(cell, Cell::Steam))
        .count();
    println!("Number of steam cell: {}", n_steam);

    // Find all the faces that touches steam
    let exposed_to_steam: usize = map
        .iter()
        .filter(|(_, cell)| matches!(cell, Cell::Rock))
        .map(|(cube, _)| {
            // println!("Point: {:?}", &cube);
            map.iter()
                // .inspect(|i| { dbg!(i); })
                .filter(|(point, cell)| {
                    matches!(cell, Cell::Steam) && point.adjacent(&cube)
                })
                .count()
        })
        .sum();

    println!("Faces exposed to steam: {}", exposed_to_steam);

    Ok(())
}
