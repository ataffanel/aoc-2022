use std::{collections::BinaryHeap, fs::File, io::Read};

fn main() {
    println!("Hello, world!");
    let mut input = File::open("input").unwrap();
    let input = {
        let mut str = String::new();
        input.read_to_string(&mut str).unwrap();
        str
    };

    let mut sums = BinaryHeap::new();
    let mut sum = 0;

    for line in input.lines() {
        if let Ok(calori) = line.parse::<i64>() {
            sum += calori;
        } else {
            sums.push(sum);
            sum = 0;
        }
    }
    sums.push(sum);

    println!(
        "The elves with the most calories has: {}",
        sums.peek().unwrap()
    );

    let top_3 = sums.pop().unwrap() + sums.pop().unwrap() + sums.pop().unwrap();

    println!(
        "The 3 elves with the most calories have a total of: {}",
        top_3
    );
}
