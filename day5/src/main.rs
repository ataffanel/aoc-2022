use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::Result;

#[derive(Debug)]
struct Move {
    times: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn execute(&self, stacks: &mut [Vec<char>]) {
        for _ in 0..self.times {
            let c = stacks[self.from - 1].pop().unwrap();
            stacks[self.to - 1].push(c);
        }
    }

    fn execute_all_at_once(&self, stacks: &mut [Vec<char>]) {
        let mut crates = Vec::new();
        for _ in 0..self.times {
            crates.push(stacks[self.from - 1].pop().unwrap());
        }
        crates.reverse();
        stacks[self.to - 1].append(&mut crates);
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inputs = s.split(' ').skip(1).step_by(2);
        Ok(Self {
            times: inputs.next().unwrap().parse()?,
            from: inputs.next().unwrap().parse()?,
            to: inputs.next().unwrap().parse()?,
        })
    }
}

fn main() -> Result<()> {
    let moves = BufReader::new(File::open("input").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            Move::from_str(&line).unwrap()
        });

    // let mut stacks = [
    //     vec!['Z', 'N'],
    //     vec!['M', 'C', 'D'],
    //     vec!['P'],
    // ];

    let mut stacks = [
        vec!['W', 'M', 'F', 'L'],
        vec!['B', 'Z', 'V', 'M', 'F'],
        vec!['H', 'V', 'R', 'S', 'L', 'Q'],
        vec!['F', 'S', 'V', 'Q', 'P', 'M', 'T', 'J'],
        vec!['L', 'S', 'W'],
        vec!['F', 'V', 'P', 'M', 'R', 'J', 'W'],
        vec!['J', 'Q', 'C', 'P', 'N', 'R', 'F'],
        vec!['V', 'H', 'P', 'S', 'Z', 'W', 'R', 'B'],
        vec!['B', 'M', 'J', 'C', 'G', 'H', 'Z', 'W'],
    ];

    for m in moves {
        m.execute(&mut stacks);
    }

    for mut stack in stacks {
        print!("{}", stack.pop().unwrap());
    }
    println!("");

    let moves = BufReader::new(File::open("input").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            Move::from_str(&line).unwrap()
        });

    let mut stacks = [
        vec!['W', 'M', 'F', 'L'],
        vec!['B', 'Z', 'V', 'M', 'F'],
        vec!['H', 'V', 'R', 'S', 'L', 'Q'],
        vec!['F', 'S', 'V', 'Q', 'P', 'M', 'T', 'J'],
        vec!['L', 'S', 'W'],
        vec!['F', 'V', 'P', 'M', 'R', 'J', 'W'],
        vec!['J', 'Q', 'C', 'P', 'N', 'R', 'F'],
        vec!['V', 'H', 'P', 'S', 'Z', 'W', 'R', 'B'],
        vec!['B', 'M', 'J', 'C', 'G', 'H', 'Z', 'W'],
    ];

    for m in moves {
        m.execute_all_at_once(&mut stacks);
    }

    for mut stack in stacks {
        print!("{}", stack.pop().unwrap());
    }
    println!("");

    Ok(())
}
