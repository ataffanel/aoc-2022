use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let forest: Vec<Vec<u8>> = read_to_string("input")?
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - '0' as u8).collect())
        .collect();

    let width = forest.len();
    let height = forest.get(0).unwrap().len();

    let mut visible_trees = 2 * width + 2 * height - 4;
    let mut best_scenic = 0;

    for x in 1..width - 1 {
        for y in 1..height - 1 {
            let tree_height = forest.get(x).unwrap().get(y).unwrap();
            let line = forest.get(x).unwrap().clone();
            let column: Vec<_> = forest.iter().map(|line| *line.get(y).unwrap()).collect();

            let mut left = line.split_at(y).0.to_vec();
            let mut right = line.split_at(y + 1).1.to_vec();
            let mut top = column.split_at(x).0.to_vec();
            let mut bottom = column.split_at(x + 1).1.to_vec();

            let mut scenic_left = left.iter().rev().take_while(|v| tree_height > v).count();
            if scenic_left < left.len() {
                scenic_left += 1;
            }
            let mut scenic_right = right.iter().take_while(|v| tree_height > v).count();
            if scenic_right < right.len() {
                scenic_right += 1;
            }
            let mut scenic_top = top.iter().rev().take_while(|v| tree_height > v).count();
            if scenic_top < top.len() {
                scenic_top += 1;
            }
            let mut scenic_bottom = bottom.iter().take_while(|v| tree_height > v).count();
            if scenic_bottom < bottom.len() {
                scenic_bottom += 1;
            }

            let score = scenic_left * scenic_right * scenic_top * scenic_bottom;

            if score > best_scenic {
                best_scenic = score;
            }
            // println!("{} {} {} {} {}", scenic_top, scenic_left, scenic_bottom, scenic_right, score);

            left.sort();
            right.sort();
            top.sort();
            bottom.sort();

            if tree_height > left.last().unwrap()
                || tree_height > right.last().unwrap()
                || tree_height > top.last().unwrap()
                || tree_height > bottom.last().unwrap()
            {
                // println!("{} {} height {} is visible", x, y, tree_height);
                visible_trees += 1;
            }
        }
    }

    println!(
        "Visible trees: {}, best scenic score: {}",
        visible_trees, best_scenic
    );
    Ok(())
}
