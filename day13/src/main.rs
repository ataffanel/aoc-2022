#![feature(iter_array_chunks)]

use std::{cmp::Ordering, fmt::Debug, fs, str::FromStr};

#[derive(Clone, PartialEq, Eq, Ord)]
enum Element {
    Integer(u32),
    List(Vec<Element>),
}

impl FromStr for Element {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') {
            let mut raw_list = s.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
            let mut list = Vec::new();
            while !raw_list.is_empty() {
                // Get the position of the next "level 0" comma
                let mut level = 0;
                let next_coma: usize = raw_list
                    .chars()
                    .take_while(|c| match c {
                        '[' => {
                            level += 1;
                            true
                        }
                        ']' => {
                            level -= 1;
                            true
                        }
                        ',' if level == 0 => false,
                        _ => true,
                    })
                    .count();

                // Split, parse and push!
                let (element, rest) = raw_list.split_at(next_coma);
                raw_list = rest.strip_prefix(",").unwrap_or(rest);
                list.push(element.parse()?);
            }

            Ok(Element::List(list))
        } else {
            Ok(Element::Integer(s.parse()?))
        }
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(n) => f.write_str(&format!("{}", n))?,
            Self::List(list) => {
                f.write_str("[")?;
                for element in list {
                    f.write_str(&format!("{:?},", element))?;
                }
                f.write_str("]")?
            }
        }

        Ok(())
    }
}

impl PartialOrd<Element> for Element {
    fn partial_cmp(&self, other: &Element) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Element::Integer(me), Element::Integer(other)) => me.partial_cmp(other),
            (Element::List(me), Element::List(other)) => {
                for i in 0..me.len().min(other.len()) {
                    let comp = me[i].partial_cmp(&other[i]);
                    if !matches!(comp, Some(Ordering::Equal)) {
                        let result = comp;
                        return result;
                    }
                }

                if me.len() > other.len() {
                    Some(Ordering::Greater)
                } else if me.len() < other.len() {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (Element::Integer(_), Element::List(_)) => {
                let me = Element::List(vec![self.clone()]);
                me.partial_cmp(other)
            }
            (Element::List(_), Element::Integer(_)) => {
                let other = Element::List(vec![other.clone()]);
                self.partial_cmp(&other)
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input")?;
    let elements = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Element>().unwrap());

    let sum_of_well_ordered: usize = elements
        .array_chunks()
        .inspect(|[left, right]| println!("\n{:?} Vs. {:?}", left, right))
        .enumerate()
        .filter(|(_, [left, right])| left <= right)
        .map(|(i, _)| i + 1)
        .inspect(|i| println!("{}", i))
        .sum();

    println!("Sum of of well ordered: {}", sum_of_well_ordered);

    let input = fs::read_to_string("input")?;
    let mut elements: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Element>().unwrap())
        .collect();

    elements.push("[[2]]".parse()?);
    elements.push("[[6]]".parse()?);

    elements.sort();

    let two_position = elements
        .iter()
        .position(|e| *e == "[[2]]".parse().unwrap())
        .unwrap()
        + 1;
    let six_position = elements
        .iter()
        .position(|e| *e == "[[6]]".parse().unwrap())
        .unwrap()
        + 1;

    println!("Decoder key: {}", two_position * six_position);

    Ok(())
}
