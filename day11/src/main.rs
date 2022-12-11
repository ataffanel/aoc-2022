use std::{
    collections::HashMap,
    fs,
    str::{FromStr, Lines},
};

#[derive(Debug)]
enum Operand<T> {
    Old,
    Some(T),
}

impl<T> FromStr for Operand<T>
where
    T: FromStr,
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Operand::Old),
            _ => Ok(Operand::Some(s.parse()?)),
        }
    }
}

impl<T> Operand<T>
where
    T: Copy,
{
    fn get(&self, old: T) -> T {
        match self {
            Operand::Old => old,
            Operand::Some(value) => *value,
        }
    }
}

#[derive(Debug)]
struct Operation {
    a: Operand<u32>,
    b: Operand<u32>,
    plus: bool,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elements = s.split(" ").skip(2); // We skip "new ="
        let a = elements.next().unwrap().parse()?;
        let plus = match elements.next().unwrap() {
            "+" => true,
            "*" => false,
            _ => return Err(anyhow::Error::msg("Operand incorect")),
        };
        let b = elements.next().unwrap().parse()?;

        Ok(Operation { a, b, plus })
    }
}

impl Operation {
    fn execute(&self, old: u32) -> u32 {
        let a = self.a.get(old);
        let b = self.b.get(old);
        match self.plus {
            false => a * b,
            true => a + b,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test_div_by: u32,
    send_if_true: usize,
    send_if_false: usize,
}

fn item_of_next_line(lines: &mut Lines) -> String {
    lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim_start_matches(' ')
        .into()
}

impl Monkey {
    fn new_from_lines(lines: &mut Lines) -> Option<Self> {
        if let None = lines.next() {
            return None;
        }

        let items = item_of_next_line(lines)
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();
        let operation = item_of_next_line(lines).parse().unwrap();
        let test_div_by = item_of_next_line(lines)
            .split(' ')
            .nth(2)
            .unwrap()
            .parse()
            .unwrap();
        let send_if_true = item_of_next_line(lines)
            .split(' ')
            .nth(3)
            .unwrap()
            .parse()
            .unwrap();
        let send_if_false = item_of_next_line(lines)
            .split(' ')
            .nth(3)
            .unwrap()
            .parse()
            .unwrap();

        // 'Eat' the empty line between monkeys
        lines.next();

        Some(Monkey {
            items,
            operation,
            test_div_by,
            send_if_true,
            send_if_false,
        })
    }

    fn execute(&mut self) -> Vec<(usize, u32)> {
        let mut transfers = Vec::new();
        for item in self.items.iter() {
            let mut worry = self.operation.execute(*item);
            worry /= 3;
            if worry % self.test_div_by == 0 {
                transfers.push((self.send_if_true, worry));
            } else {
                transfers.push((self.send_if_false, worry));
            }
        }

        self.items = Vec::new();

        transfers
    }
}

fn main() -> anyhow::Result<()> {
    let mut input = fs::read_to_string("input")?;
    let mut lines = input.lines();

    let mut monkeys: Vec<_> = std::iter::from_fn(|| Monkey::new_from_lines(&mut lines)).collect();
    let mut n_inspections: Vec<_> = monkeys.iter().map(|_| 0).collect();

    // dbg!(monkeys);

    // Simulate the monkey throw!

    for round in 0..20 {
        println!("Round {}:", round + 1);

        for i in 0..monkeys.len() {
            *n_inspections.get_mut(i).unwrap() += monkeys[i].items.len();
            let transfers = monkeys[i].execute();

            for (index, item) in transfers.iter() {
                monkeys[*index].items.push(*item);
            }
        }

        for (i, monkey) in monkeys.iter().enumerate() {
            println!(" Monkey {}: {:?}", i + 1, monkey.items);
        }
    }

    dbg!(&n_inspections);

    n_inspections.sort();
    n_inspections.reverse();

    let business = n_inspections[0] * n_inspections[1];

    println!("Monkey business: {}", business);

    Ok(())
}
