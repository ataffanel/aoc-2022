use std::{
    fs,
    str::{FromStr, Lines}, ops::{Mul, Add, DivAssign, Rem},
};

#[derive(Debug)]
enum Operand<T> {
    Old,
    Some(T),
}

impl<T> FromStr for Operand<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug + Sync + Send + std::error::Error
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Operand::Old),
            _ => Ok(Operand::Some(s.parse().unwrap())),
        }
    }
}

impl<T> Operand<T>
    where T: Clone
{
    fn get(&self, old: T) -> T {
        match self {
            Operand::Old => old,
            Operand::Some(value) => value.clone(),
        }
    }
}

#[derive(Debug)]
struct Operation<T> {
    a: Operand<T>,
    b: Operand<T>,
    plus: bool,
}

impl <T> FromStr for Operation<T>
    where T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug + Sync + Send + std::error::Error
{
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

impl <T> Operation<T>
    where T: Clone + Mul<Output = T> + Add<Output = T>
{
    fn execute(&self, old: T) -> T {
        let a = self.a.get(old.clone());
        let b = self.b.get(old);
        match self.plus {
            false => a * b,
            true => a + b,
        }
    }
}

#[derive(Debug)]
struct Monkey<T> {
    items: Vec<T>,
    operation: Operation<T>,
    test_div_by: T,
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

impl <T> Monkey<T>
    where T: Clone + FromStr + std::fmt::Debug + Mul<Output = T> + Add<Output = T> + DivAssign + From<u32> + Rem,
        <T as FromStr>::Err: std::fmt::Debug + Sync + Send + std::error::Error,
        <T as Rem>::Output: PartialEq<T>
        
{
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

    fn execute(&mut self) -> Vec<(usize, T)> {
        let mut transfers = Vec::new();
        for item in self.items.iter() {
            let mut worry = self.operation.execute(item.clone());
            // worry /= 3.into();
            if worry.clone() % self.test_div_by.clone() == 0.into() {
                transfers.push((self.send_if_true, worry));
            } else {
                transfers.push((self.send_if_false, worry));
            }
        }

        self.items = Vec::new();

        transfers
    }
}

// This can (but does not have to) be `num::BigUint` or `rug::Integer`
// u64 takes 0.6s while the two bigint implementation are at about 0.95s on my machine
type NumImplementation = u64;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input")?;
    let mut lines = input.lines();

    let mut monkeys: Vec<Monkey<NumImplementation>> = std::iter::from_fn(|| Monkey::new_from_lines(&mut lines)).collect();
    let mut n_inspections: Vec<_> = monkeys.iter().map(|_| 0).collect();

    let common_divider: NumImplementation = monkeys.iter().map(|m| m.test_div_by.clone()).fold(1u32.into(), |acc, val| acc * val);
    println!("Common divider: {}", &common_divider);

    for round in 0..10_000 {
        println!("Round {}:", round + 1);

        for i in 0..monkeys.len() {
            *n_inspections.get_mut(i).unwrap() += monkeys[i].items.len();
            let transfers = monkeys[i].execute();

            for (index, item) in transfers.iter() {
                monkeys[*index].items.push(item.clone());
            }
        }

        // Keep worry in check!
        for monkey in monkeys.iter_mut() {
            for item in monkey.items.iter_mut() {
                *item %= common_divider.clone()
            }
        }
    }

    dbg!(&n_inspections);

    n_inspections.sort();
    n_inspections.reverse();

    let business = n_inspections[0] * n_inspections[1];

    println!("Monkey business: {}", business);

    Ok(())
}
