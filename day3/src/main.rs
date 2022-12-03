#![feature(iter_array_chunks)]
use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Item {
    id: char,
}

impl Item {
    fn priority(&self) -> u32 {
        match self.id {
            'a'..='z' => self.id as u32 - 'a' as u32 + 1,
            'A'..='Z' => self.id as u32 - 'A' as u32 + 27,
            _ => panic!("Bad input"),
        }
    }
}

#[derive(Debug)]
struct Compartment {
    content: Vec<Item>,
}

impl Compartment {
    fn new(input: &str) -> Self {
        Self {
            content: input.chars().map(|c| Item { id: c }).collect(),
        }
    }
}

#[derive(Debug)]
struct Rucksack {
    raw: String,
    left: Compartment,
    right: Compartment,
}

impl Rucksack {
    fn new(left: &str, right: &str) -> Self {
        let mut raw = left.to_owned();
        raw.push_str(right);
        Self {
            raw,
            left: Compartment::new(left),
            right: Compartment::new(right),
        }
    }

    fn common_items(&self) -> Vec<Item> {
        self.left
            .content
            .clone()
            .into_iter()
            .filter(|left_item| {
                self.right
                    .content
                    .iter()
                    .filter(|right_item| &left_item == right_item)
                    .count()
                    != 0
            })
            .collect()
    }
}

struct Group {
    rucksacks: [Rucksack; 3],
}

impl Group {
    fn new(rucksacks: [Rucksack; 3]) -> Self {
        Self { rucksacks }
    }

    fn tag_item(&self) -> Item {
        let id = self.rucksacks[0]
            .raw
            .chars()
            .filter(|c| self.rucksacks[1].raw.contains(*c) && self.rucksacks[2].raw.contains(*c))
            .next()
            .unwrap();

        Item { id }
    }
}

fn main() -> Result<()> {
    let input = BufReader::new(File::open("input")?).lines().map(|line| {
        let line = line.unwrap();
        let (left, right) = line.split_at(line.len() / 2);
        Rucksack::new(left, right)
    });

    let sum_of_commom: u32 = input
        .map(|rucksack| rucksack.common_items().first().unwrap().priority())
        .sum();

    println!("Sum of the common items: {}", sum_of_commom);

    let input = BufReader::new(File::open("input")?).lines().map(|line| {
        let line = line.unwrap();
        let (left, right) = line.split_at(line.len() / 2);
        Rucksack::new(left, right)
    });

    let groups = input.array_chunks().map(|group| Group::new(group));

    let sum_of_tags: u32 = groups.map(|g| g.tag_item().priority()).sum();

    println!("Sum of tags: {}", sum_of_tags);

    Ok(())
}
