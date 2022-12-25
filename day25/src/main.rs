use std::fs;

use anyhow::Result;

fn parse_snafu(s: &str) -> i64 {
    let mut num = 0;
    let mut chars = s.chars();

    let size = s.len() as u32;

    for pos in (0..size).rev() {
        let n = match chars.next().unwrap() {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("Bad input"),
        };

        num += n * 5i64.pow(pos);
    }

    num
}

fn print_snafu(mut num: i64) {
    let mut res = String::new();

    while num > 0 {
        let mut rem = num % 5;
        num = num / 5;

        if rem == 3 {
            rem = -2;
            num += 1;
        }
        if rem == 4 {
            rem = -1;
            num += 1;
        }

        res.push(match rem {
            -2 => '=',
            -1 => '-', 
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!(),
        });
    }
    
    let res: String = res.chars().rev().collect();
    println!("{}", res);
}

fn main() -> Result<()>{
    let numbers: Vec<i64> = fs::read_to_string("input")?.lines().map(|line| parse_snafu(line)).collect();

    let sum = numbers.iter().sum::<i64>();
    println!("Sum: {}", sum);
    print_snafu(sum);

    Ok(())
}
