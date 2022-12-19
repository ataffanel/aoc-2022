use std::{
    collections::{BTreeSet, VecDeque},
    ops::{Sub, SubAssign},
    str::FromStr, fs,
};

#[derive(Debug)]
struct Blueprint {
    ore_bot: Stash,
    clay_bot: Stash,
    obsidian_bot: Stash,
    geode_bot: Stash,
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elements = s.split(':').nth(1).unwrap().split('.');

        Ok(Blueprint {
            ore_bot: elements.next().unwrap().split("costs ").nth(1).unwrap().parse()?,
            clay_bot: elements.next().unwrap().split("costs ").nth(1).unwrap().parse()?,
            obsidian_bot: elements.next().unwrap().split("costs ").nth(1).unwrap().parse()?,
            geode_bot: elements.next().unwrap().split("costs ").nth(1).unwrap().parse()?,
        })
    }
}

impl Blueprint {
    fn can_build_ore_bot(&self, stash: &Stash) -> bool {
        stash.ore >= self.ore_bot.ore
            && stash.clay >= self.ore_bot.clay
            && stash.obsidian >= self.ore_bot.obsidian
            && stash.geode >= self.ore_bot.geode
    }

    fn can_build_clay_bot(&self, stash: &Stash) -> bool {
        stash.ore >= self.clay_bot.ore
            && stash.clay >= self.clay_bot.clay
            && stash.obsidian >= self.clay_bot.obsidian
            && stash.geode >= self.clay_bot.geode
    }

    fn can_build_obsidian_bot(&self, stash: &Stash) -> bool {
        stash.ore >= self.obsidian_bot.ore
            && stash.clay >= self.obsidian_bot.clay
            && stash.obsidian >= self.obsidian_bot.obsidian
            && stash.geode >= self.obsidian_bot.geode
    }

    fn can_build_geode_bot(&self, stash: &Stash) -> bool {
        stash.ore >= self.geode_bot.ore
            && stash.clay >= self.geode_bot.clay
            && stash.obsidian >= self.geode_bot.obsidian
            && stash.geode >= self.geode_bot.geode
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Stash {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl FromStr for Stash {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stash = Stash::default();
        for item in s.split(" and ") {
            let mut item = item.split(" ");
            let quantity = item.next().unwrap().parse()?;
            let element = item.next().unwrap();
            match element {
                "ore" => stash.ore = quantity,
                "clay" => stash.clay = quantity,
                "obsidian" => stash.obsidian = quantity,
                "geode" => stash.geode = quantity,
                _ => panic!("Bad input!"),
            }
        }
        Ok(stash)
    }
}

impl Sub<Stash> for Stash {
    type Output = Stash;

    fn sub(self, rhs: Stash) -> Self::Output {
        Stash {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl SubAssign for Stash {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    ore_bot: usize,
    clay_bot: usize,
    obsidian_bot: usize,
    geode_bot: usize,
    stash: Stash,
}

impl Default for State {
    fn default() -> Self {
        Self {
            ore_bot: 1,
            clay_bot: Default::default(),
            obsidian_bot: Default::default(),
            geode_bot: Default::default(),
            stash: Default::default(),
        }
    }
}

impl State {
    fn run_bots(&mut self) {
        self.stash.ore += self.ore_bot;
        self.stash.clay += self.clay_bot;
        self.stash.obsidian += self.obsidian_bot;
        self.stash.geode += self.geode_bot;
    }
}

fn test_blueprint(blueprint: &Blueprint) -> usize {
    println!("\nRunning bluepring:");
    dbg!(blueprint);
    // let mut to_visit = VecDeque::new();
    let mut to_visit = BTreeSet::new();
    to_visit.insert(State::default());

    for round in 0..24 {
        println!("Round {}, {} states to inspect", round, to_visit.len());
        let mut new_states = BTreeSet::new();
        // Empty the queue once on each round
        for mut state in to_visit.iter().cloned() {
            // First, run the bots

            // dbg!(&state);

            // Now the branches, try to build all the possible bots and push the resulting state in the queue
            if blueprint.can_build_geode_bot(&state.stash){
                let mut state = state.clone();
                state.stash -= blueprint.geode_bot;

                state.run_bots();

                state.geode_bot += 1;
                new_states.insert(state);
            } else if blueprint.can_build_obsidian_bot(&state.stash) {
                let mut state = state.clone();
                state.stash -= blueprint.obsidian_bot;

                state.run_bots();

                state.obsidian_bot += 1;
                new_states.insert(state);
            } else {
                if blueprint.can_build_ore_bot(&state.stash) {
                    let mut state = state.clone();
                    state.stash -= blueprint.ore_bot;

                    state.run_bots();

                    state.ore_bot += 1;
                    new_states.insert(state);
                }

                if blueprint.can_build_clay_bot(&state.stash) {
                    let mut state = state.clone();
                    state.stash -= blueprint.clay_bot;

                    state.run_bots();

                    state.clay_bot += 1;
                    new_states.insert(state);
                }
            }

            // Building nothing is always a possible outcome
            state.run_bots();
            new_states.insert(state);
            
        }

        dbg!(&new_states.first());

        to_visit = new_states;
        println!(
            "Ammount of geodes: {}",
            to_visit
                .iter()
                .map(|state| state.stash.geode)
                .max()
                .unwrap()
        );
    }

    // Now contains all the states after 24 round
    // Let find the max amount of geode reached
    to_visit
        .iter()
        .map(|state| state.stash.geode)
        .max()
        .unwrap()
}

fn main() -> anyhow::Result<()>{
    #[rustfmt::skip]
    let blueprints = [
        Blueprint {
            ore_bot: Stash {ore: 4, ..Default::default()},
            clay_bot: Stash {ore: 2, ..Default::default()},
            obsidian_bot: Stash {ore: 3, clay: 14, ..Default::default()},
            geode_bot: Stash { ore: 2, obsidian: 7, ..Default::default() }
        },
        Blueprint {
            ore_bot: Stash {ore: 2, ..Default::default()},
            clay_bot: Stash {ore: 3, ..Default::default()},
            obsidian_bot: Stash {ore: 3, clay: 8, ..Default::default()},
            geode_bot: Stash { ore: 3, obsidian: 12, ..Default::default() }
        },
    ];

    let blueprints: Vec<Blueprint> = fs::read_to_string("input")?.lines().map(|line| line.parse().unwrap()).collect();

    let quality_sum = blueprints
        .iter()
        .enumerate()
        .map(|(n, blueprint)| (n + 1) * test_blueprint(blueprint))
        .inspect(|q| println!("Quality: {}", q))
        .sum::<usize>();

    println!("Sum of the blueprint: {}", quality_sum);

    Ok(())
}
