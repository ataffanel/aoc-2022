use std::fs;

struct Message {
    array: Vec<(usize, isize)>,
}

impl Message {
    fn new(input: &[isize]) -> Message {
        Message {
            array: input.iter().cloned().enumerate().collect(),
        }
    }

    fn move_left(&mut self, id: usize) {
        let position = self.array.iter().position(|(n, _)| *n == id).unwrap();

        let value = self.array.remove(position);
        if position == 0 {
            self.array.insert(self.array.len() - 1, value);
        } else {
            self.array.insert(position - 1, value);
        }
    }

    fn move_right(&mut self, id: usize) {
        let position = self.array.iter().position(|(n, _)| *n == id).unwrap();

        let value = self.array.remove(position);
        if position >= self.array.len() {
            self.array.insert(1, value);
        } else {
            self.array.insert(position + 1, value);
        }
    }

    fn shuffle(&mut self) {
        for i in 0..self.array.len() {
            let pos = self.array.iter().position(|(n, _)| *n == i).unwrap();

            // println!("{:?}", &self.array);
            let value = self.array[pos].1;
            if value > 0 {
                for _ in 0..(value % (self.array.len() - 1) as isize) {
                    self.move_right(i);
                }
            } else {
                for _ in 0..((-value) % (self.array.len() - 1) as isize) {
                    self.move_left(i);
                    // println!("{:?}", &self.array);
                }
            }
        }

        // println!("{:?}", &self.array);
        // println!();
    }

    /// Returns an iterator that infinitly loop arround the values
    /// Makes it possible to easily get the "1000th value after 0"
    fn iter(&'_ self) -> Iter<'_> {
        Iter {
            message: self,
            id: 0,
        }
    }
}

struct Iter<'a> {
    message: &'a Message,
    id: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.id;
        self.id = (self.id + 1) % self.message.array.len();
        Some(self.message.array[pos].1)
    }
}

fn main() -> anyhow::Result<()> {
    println!("Part 1:");

    let input: Vec<isize> = fs::read_to_string("input")?
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut message = Message::new(&input);

    message.shuffle();

    let coordinate_1 = message.iter().skip_while(|v| *v != 0).nth(1000).unwrap();
    let coordinate_2 = message.iter().skip_while(|v| *v != 0).nth(2000).unwrap();
    let coordinate_3 = message.iter().skip_while(|v| *v != 0).nth(3000).unwrap();

    println!(
        "Coordinates: {} {} {}",
        coordinate_1, coordinate_2, coordinate_3
    );

    println!("Result: {}", coordinate_1 + coordinate_2 + coordinate_3);

    println!();
    println!("Part 2:");

    let input: Vec<isize> = fs::read_to_string("input")?
        .lines()
        .map(|line| line.parse::<isize>().unwrap() * 811589153)
        .collect();

    let mut message = Message::new(&input);

    for _ in 0..10 {
        message.shuffle();
    }

    let coordinate_1 = message.iter().skip_while(|v| *v != 0).nth(1000).unwrap();
    let coordinate_2 = message.iter().skip_while(|v| *v != 0).nth(2000).unwrap();
    let coordinate_3 = message.iter().skip_while(|v| *v != 0).nth(3000).unwrap();

    println!(
        "Coordinates: {} {} {}",
        coordinate_1, coordinate_2, coordinate_3
    );
    println!("Result: {}", coordinate_1 + coordinate_2 + coordinate_3);

    Ok(())
}
