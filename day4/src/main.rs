use std::{str::FromStr, io::{BufReader, BufRead}, fs::File};

use anyhow::Result;

#[derive(Debug)]
struct Pair {
    start: u32,
    end: u32,
}

impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = s.split("-");

        Ok(
            Self {
                start: items.next().unwrap().parse()?,
                end: items.next().unwrap().parse()?,
            }
        )
    }
}

#[derive(Debug)]
struct Sections (Pair, Pair);

impl Sections {
    fn fully_overlaps(&self) -> bool {
        (self.0.start <= self.1.start && self.0.end >= self.1.end) ||
        (self.0.start >= self.1.start && self.0.end <= self.1.end)
    }

    fn overlaps(&self) -> bool {
        let overlaps = (self.0.start..=self.0.end).contains(&self.1.start) || (self.1.start..=self.1.end).contains(&self.0.start);
        dbg!(self, overlaps);
        overlaps
    }
}

impl FromStr for Sections {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = s.split(",");
        Ok(Self (
            Pair::from_str(pairs.next().unwrap())?,
            Pair::from_str(pairs.next().unwrap())?,
        ))
    }
}

fn main() -> Result<()>{
    let input = BufReader::new(File::open("input")?)
        .lines()
        .map(|line| Sections::from_str(&line.unwrap()).unwrap());

    let full_overlap = input.filter(|section| section.fully_overlaps()).count();

    let input = BufReader::new(File::open("input")?)
        .lines()
        .map(|line| Sections::from_str(&line.unwrap()).unwrap());

    let some_overlap = input.filter(|section| section.overlaps()).count();

    println!("Overlaps: {}, Some overlaps: {}", full_overlap, some_overlap);
    Ok(())
}
