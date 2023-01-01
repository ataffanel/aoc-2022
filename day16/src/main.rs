use regex::Regex;
use std::{
    collections::{BinaryHeap, VecDeque},
    fs,
    str::FromStr,
};

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
    distances: Vec<Vec<usize>>, // Matrix of distances[from][to] valves
}

#[derive(Debug, Clone)]
struct State {
    valves_opened: Vec<bool>,
    current_valve: usize,
    released: usize,
    time: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.score() == other.score()
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score().partial_cmp(&other.score())
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().total_cmp(&other.score())
    }
}

impl State {
    fn score(&self) -> f64 {
        self.released as f64 / self.time as f64
    }

    fn travel(&mut self, to_valve: usize, travel_time: usize, valves: &Vec<Valve>) {
        self.released += self.release_rate(valves) * (travel_time + 1);
        self.current_valve = to_valve;
        self.time += travel_time + 1; // We need 1 minute to open the valve!
        self.valves_opened[to_valve] = true;
    }

    fn release_rate(&self, valves: &[Valve]) -> usize {
        valves
            .iter()
            .enumerate()
            .filter(|(i, _)| self.valves_opened[*i])
            .map(|(_, v)| v.rate)
            .sum()
    }
}

fn find_valve_distances(from: usize, valves: &Vec<Valve>) -> Vec<usize> {
    let mut distances = vec![0; valves.len()];
    let mut to_visit = VecDeque::new();

    to_visit.push_front(from);

    while !to_visit.is_empty() {
        let valve_id = to_visit.pop_back().unwrap();

        for next in valves[valve_id].next.iter() {
            let next_id = valves.iter().position(|v| &v.name == next).unwrap();

            if next_id != from && distances[next_id] == 0 {
                distances[next_id] = distances[valve_id] + 1;
                to_visit.push_front(next_id);
            }
        }
    }

    distances
}

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valves: Vec<Valve> = s
            .lines()
            .map(|line| {
                let valve: Valve = line.parse().unwrap();
                valve
            })
            .collect();

        let distances = (0..valves.len())
            .map(|v| find_valve_distances(v, &valves))
            .collect();

        Ok(Cave { valves, distances })
    }
}

impl Cave {
    fn find_max_release(&self, run_time: usize) -> usize {
        let aa_valve = self.valves.iter().position(|v| &v.name == "AA").unwrap();

        let mut states: BinaryHeap<State> = [State {
            valves_opened: vec![false; self.valves.len()],
            current_valve: aa_valve,
            released: 0,
            time: 0,
        }]
        .iter()
        .cloned()
        .collect();

        let mut max_release = 0;
        let mut max_time = 0;

        while !states.is_empty() {
            states = states
                .iter()
                .flat_map(|state| {
                    // update max_release with this state at run time
                    let this_max = ((run_time - state.time) * state.release_rate(&self.valves))
                        + state.released;
                    max_release = max_release.max(this_max);
                    max_time = max_time.max(state.time);

                    // Visit all possible valves as long as we do not go over the runtime
                    state
                        .valves_opened
                        .iter()
                        .enumerate()
                        .filter(|(_, opened)| !*opened)
                        .map(|(valve_id, _)| {
                            let mut state = state.clone();

                            let travel_time = self.distances[state.current_valve][valve_id];
                            state.travel(valve_id, travel_time, &self.valves);

                            state
                        })
                        .filter(|state| state.time < run_time)
                        .filter(|state| self.valves[state.current_valve].rate > 0)
                })
                .collect();

            println!(
                "{} states, max release: {}, max_time: {}",
                states.len(),
                max_release,
                max_time
            );
        }

        max_release
    }
}

fn main() -> anyhow::Result<()> {
    let cave: Cave = fs::read_to_string("input").unwrap().parse()?;

    println!("Max release: {}", cave.find_max_release(30));

    Ok(())
}
