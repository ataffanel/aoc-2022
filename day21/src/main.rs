use std::{collections::HashMap, fs, str::FromStr};

use anyhow::{Error, Result};

#[derive(Debug)]
enum Element {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Number(i64),
}

impl FromStr for Element {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elements = s.split(' ');

        let first = elements.next().unwrap().to_owned();

        if let Ok(num) = first.parse::<i64>() {
            Ok(Element::Number(num))
        } else {
            let operation = elements.next().unwrap();
            let second = elements.next().unwrap().to_owned();

            match operation {
                "+" => Ok(Element::Add(first, second)),
                "-" => Ok(Element::Sub(first, second)),
                "*" => Ok(Element::Mul(first, second)),
                "/" => Ok(Element::Div(first, second)),
                _ => Err(anyhow::Error::msg("Bad input format!")),
            }
        }
    }
}

impl Element {
    fn calculate(&self, first: i64, second: i64) -> anyhow::Result<i64> {
        match self {
            Element::Add(_, _) => Ok(first + second),
            Element::Sub(_, _) => Ok(first - second),
            Element::Mul(_, _) => Ok(first * second),
            Element::Div(_, _) => Ok(first / second),
            Element::Number(_) => Err(Error::msg("Not an operation.")),
        }
    }

    fn is_operation(&self) -> bool {
        !matches!(self, Self::Number(_))
    }

    fn get_orerands(&self) -> Result<(String, String)> {
        match self {
            Element::Add(f, s) => Ok((f.to_owned(), s.to_owned())),
            Element::Sub(f, s) => Ok((f.to_owned(), s.to_owned())),
            Element::Mul(f, s) => Ok((f.to_owned(), s.to_owned())),
            Element::Div(f, s) => Ok((f.to_owned(), s.to_owned())),
            Element::Number(_) => Err(Error::msg("Not an operation.")),
        }
    }

    fn get_number(&self) -> Result<i64> {
        match self {
            Element::Number(n) => Ok(*n),
            _ => Err(Error::msg("Not an number.")),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut input: HashMap<String, Element> = fs::read_to_string("input")?
        .lines()
        .map(|line| {
            let mut elements = line.split(": ");
            (
                elements.next().unwrap().to_owned(),
                elements.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let keys: Vec<String> = input.keys().cloned().collect();

    while input["root"].is_operation() {
        for name in keys.iter() {
            if input[name].is_operation() {
                let (first, second) = input[name].get_orerands()?;
                if !input[&first].is_operation() && !input[&second].is_operation() {
                    println!("Running operatio for {}: {:?}", name, input[name]);
                    let value = input[name]
                        .calculate(input[&first].get_number()?, input[&second].get_number()?)?;
                    println!("{} is now {}", name, value);
                    input.insert(name.to_owned(), Element::Number(value));
                }
            }
        }
    }

    println!("Root: {}", input["root"].get_number()?);

    // dbg!(input);

    Ok(())
}
