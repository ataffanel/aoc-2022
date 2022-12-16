use std::{fs, str::FromStr};
use rayon::prelude::*;

enum Cell {
    Scanned,
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn distance(self, other: Position) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Sensor {
    position: Position,
    sensing_distance: isize,
    beacon: Position,
}

impl Sensor {
    fn can_sense(&self, target: Position) -> bool {
        self.position.distance(target) <= self.sensing_distance
    }
}

struct Field {
    sensors: Vec<Sensor>,
    min_sensing: Position,
    max_sensing: Position,
}

// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
impl FromStr for Field {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sensors = Vec::new();
        let mut min_sensing = Position::default();
        let mut max_sensing = Position::default();

        for line in s.lines() {
            let mut elements = line.split(": ");
            // dbg!(line);
            let mut sensor = elements
                .next()
                .unwrap()
                .strip_prefix("Sensor at ")
                .unwrap()
                .split(", ");

            let sensor = Position {
                x: sensor.next().unwrap().strip_prefix("x=").unwrap().parse()?,
                y: sensor.next().unwrap().strip_prefix("y=").unwrap().parse()?,
            };

            let mut beacon = elements
                .next()
                .unwrap()
                .strip_prefix("closest beacon is at ")
                .unwrap()
                .split(", ");
            let beacon = Position {
                x: beacon.next().unwrap().strip_prefix("x=").unwrap().parse()?,
                y: beacon.next().unwrap().strip_prefix("y=").unwrap().parse()?,
            };

            // if (sensor.x,sensor.y) != (8,7) {
            //     continue;
            // }

            let sensing_distance = sensor.distance(beacon);

            sensors.push(Sensor {
                position: sensor,
                sensing_distance,
                beacon,
            });

            if sensor.x - sensing_distance < min_sensing.x {
                min_sensing.x = sensor.x - sensing_distance;
            }
            if sensor.y - sensing_distance < min_sensing.y {
                min_sensing.y = sensor.y - sensing_distance;
            }
            if sensor.x + sensing_distance > max_sensing.x {
                max_sensing.x = sensor.x + sensing_distance;
            }
            if sensor.y + sensing_distance > max_sensing.y {
                max_sensing.y = sensor.y + sensing_distance;
            }
        }

        Ok(Field {
            sensors,
            min_sensing,
            max_sensing,
        })
    }
}

impl Field {
    fn is_sensed(&self, target: Position) -> bool {
        self.sensors
            .iter()
            .find(|sensor| sensor.can_sense(target))
            .is_some()
    }
}

fn main() -> anyhow::Result<()> {
    let input: Field = fs::read_to_string("input")?.parse()?;

    dbg!(input.min_sensing, input.max_sensing);

    let n_scanned: usize = (input.min_sensing.x..input.max_sensing.x)
        .filter(|x| input.is_sensed(Position { x: *x, y: 2000000 }))
        .count();

    for x in -4..=26 {
        if input.is_sensed(Position { x, y: 10 }) {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();

    println!("n scanned at 10: {}", n_scanned -1);

    // Searching the beacon
    (0..=4000000).into_par_iter().for_each(|y| {
        for x in 0..=4000000 {
            if !input.is_sensed(Position{x,y}) {
                dbg!(x, y);
            }
        }
        if y % 1000 == 0 {
            println!("Progress: {}%", y as f32 * 100.0f32 / 4000000.0f32);
        }
    });

    Ok(())
}
