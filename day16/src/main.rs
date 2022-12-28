
use std::{collections::BTreeSet, fs, str::FromStr};
use regex::Regex;

use rayon::prelude::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Valve {
    name: String,
    rate: usize,
    next: Vec<String>,
    opened: bool,
}

impl FromStr for Valve {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ((?:(?:[A-Z]{2}),? ?)+)$").unwrap();

        let m = re.captures(s).unwrap();

        let name = m[1].to_owned();
        let rate = m[2].parse()?;
        let next = m[3].split(", ").map(|n| n.to_owned()).collect();

        Ok(Valve {
            name,
            rate,
            next,
            opened: false,
        })
    }
}

#[derive(Debug, Clone)]
struct Cave {
    valves: Vec<Valve>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    valves_opened: Vec<bool>,
    released: usize,
    current_room: usize,
}

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valves = s
            .lines()
            .map(|line| {
                let valve: Valve = line.parse().unwrap();
                valve
            })
            .collect();

        Ok(Cave { valves })
    }
}

impl Cave {
    fn find_best_route(&self) -> usize {
        let aa_room = self.valves.iter().position(|valve| valve.name == "AA").unwrap();

        let mut states: BTreeSet<_> = [State {
            valves_opened: vec![false; self.valves.len()],
            released: 0,
            current_room: aa_room,
        }].iter().cloned().collect();

        for time in 0..30 {
            println!("Simulating time {}, {} states", time, states.len());

            let new_states = states.par_iter().cloned().map(|mut state| {
                let mut new_states = Vec::new();
                
                state.released += state
                    .valves_opened
                    .iter()
                    .enumerate()
                    .filter(|(_, opened)| **opened)
                    .map(|(i, _)| self.valves[i].rate)
                    .sum::<usize>();

                // If all valves are already opened, do nothing
                if state.valves_opened.iter().all(|opened| *opened) {
                    new_states.push(state.clone());
                    return new_states;
                }

                // Open the valve
                if !state.valves_opened[state.current_room] {
                    let mut new_state = state.clone();
                    new_state.valves_opened[state.current_room] = true;
                    new_states.push(new_state);
                }

                // Move to all possible rooms
                for room in &self.valves[state.current_room].next {
                    let mut new_state = state.clone();
                    let new_room = self.valves.iter().position(|valve| valve.name == room.to_owned()).unwrap();
                    new_state.current_room = new_room;
                    new_states.push(new_state);
                }

                new_states
            }).flatten().collect();

            states = new_states;

            // Prune state
            let max_released = states.iter().map(|state| state.released).max().unwrap();

            if states.len() > 10000000 {
                dbg!(states.len());
                dbg!(states.iter().map(|state| state.released).max());
                states.retain(|state| state.released as f64 >= max_released as f64 * 0.9f64);
                dbg!(states.len());
                dbg!(states.iter().map(|state| state.released).max());
            }
        }

        let max_released = states.iter().map(|state| state.released).max().unwrap();

        max_released
    }
}

fn main() -> anyhow::Result<()> {
    let cave: Cave = fs::read_to_string("input").unwrap().parse()?;

    println!("Max release: {}", cave.find_best_route());

    Ok(())
}
