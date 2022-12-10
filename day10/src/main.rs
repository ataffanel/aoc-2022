use std::{fs, str::FromStr};

enum Instruction {
    Add(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elements = s.split(' ');
        match elements.next() {
            Some("addx") => Ok(Instruction::Add(elements.next().unwrap().parse()?)),
            Some("noop") => Ok(Instruction::Noop),
            _ => Err(anyhow::Error::msg("Bad input")),
        }
    }
}

struct Processor {
    program: Vec<Instruction>,
    cycle: usize,
    pc: usize,
    x: i32,
    temp_x: i32,
    exec_add: bool,
}

impl Processor {
    fn new(program: Vec<Instruction>) -> Self {
        Processor {
            program,
            cycle: 0,
            pc: 0,
            x: 1,
            temp_x: 1,
            exec_add: false,
        }
    }

    fn step(&mut self) -> Option<usize> {
        self.cycle += 1;
        self.x = self.temp_x;

        match (self.exec_add, self.program.get(self.pc)) {
            (false, Some(Instruction::Add(_))) => {
                self.exec_add = true;
                Some(self.cycle)
            }
            (true, Some(Instruction::Add(n))) => {
                self.temp_x += n;
                self.pc += 1;
                self.exec_add = false;
                Some(self.cycle)
            }
            (_, Some(Instruction::Noop)) => {
                self.pc += 1;
                Some(self.cycle)
            }
            (_, None) => None,
        }
    }
}

#[derive(Default)]
struct Beam {
    position: i32,
}

impl Beam {
    fn step(&mut self) -> bool {
        self.position = (self.position + 1) % 40;
        self.position == 0
    }

    fn is_lit(&self, x: i32) -> bool {
        (x - self.position).abs() < 2
    }
}

fn main() -> anyhow::Result<()> {
    let program = fs::read_to_string("input")?
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();

    let mut processor = Processor::new(program);
    let mut beam = Beam::default();

    let trace_cycles = [20, 60, 100, 140, 180, 220];

    let mut result = 0;

    while let Some(cycle) = processor.step() {
        if trace_cycles.contains(&cycle) {
            result += cycle as i32 * processor.x;
        }

        if beam.is_lit(processor.x) {
            print!("#");
        } else {
            print!(" ");
        }

        if beam.step() {
            println!();
        }
    }
    println!();

    println!("The result is {}", result);

    Ok(())
}
