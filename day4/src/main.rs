use primitive_types::U256;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part1(rolls: &[U256], line_len: usize) -> u32 {
    rolls.iter().enumerate().fold(0, |acc, (index, value)| {
        let mut count = 0;
        for i in 0..line_len {
            let item_check = U256::from(1) << i;
            let is_roll = *value & item_check;
            if is_roll != U256::from(0) {
                let mut grid_check = item_check;
                if i > 0 {
                    grid_check |= U256::from(1) << (i - 1);
                }
                if i < line_len - 1 {
                    grid_check |= U256::from(1) << (i + 1);
                }

                let mut occupied = 0;
                for &limb in (*value & grid_check).as_ref() {
                    occupied += limb.count_ones();
                }

                if index > 0 {
                    for &limb in (rolls[index - 1] & grid_check).as_ref() {
                        occupied += limb.count_ones();
                    }
                }

                if index < rolls.len() - 1 {
                    for &limb in (rolls[index + 1] & grid_check).as_ref() {
                        occupied += limb.count_ones();
                    }
                }

                if occupied < 5 {
                    count += 1;
                }
            }
        }
        acc + count
    })
}

fn part2(rolls: &[U256], line_len: usize) -> u64 {
    let mut total = 0u64;
    let mut rolls = Vec::from(rolls);
    loop {
        let mut new_rolls = Vec::new();
        let rolls_removed = rolls.iter().enumerate().fold(0, |acc, (index, value)| {
            let mut count = 0;
            let mut remove = U256::from(0);
            for i in 0..line_len {
                let item_check = U256::from(1) << i;
                let status = *value & item_check;

                if status == U256::from(0) {
                    continue;
                }

                let mut grid_check = item_check;
                if i > 0 {
                    grid_check |= U256::from(1) << (i - 1);
                }
                if i < line_len - 1 {
                    grid_check |= U256::from(1) << (i + 1);
                }

                let mut occupied = 0;
                for &limb in (*value & grid_check).as_ref() {
                    occupied += limb.count_ones();
                }

                if index > 0 {
                    for &limb in (rolls[index - 1] & grid_check).as_ref() {
                        occupied += limb.count_ones();
                    }
                }

                if index < rolls.len() - 1 {
                    for &limb in (rolls[index + 1] & grid_check).as_ref() {
                        occupied += limb.count_ones();
                    }
                }

                if occupied < 5 {
                    count += 1;
                    remove |= item_check;
                }
            }

            new_rolls.push(*value ^ remove);

            acc + count
        });

        if rolls_removed == 0 {
            break;
        }

        total += rolls_removed;

        rolls = new_rolls;
    }
    total
}
fn parse_line(line: &str) -> U256 {
    let mut pow = U256::from(0);

    line.chars().rev().fold(U256::from(0), |acc, val| {
        let ret = acc
            | (if val == '@' {
                U256::from(1) << pow
            } else {
                U256::from(0)
            });
        pow += U256::from(1);
        ret
    })
}

fn main() -> anyhow::Result<()> {
    let input = File::open("input.txt")?;
    let input_reader = BufReader::new(input);

    let start_time = std::time::Instant::now();
    let mut line_len = 0;

    let items = input_reader
        .lines()
        .map(|line| {
            let line = line.ok()?;
            line_len = line.len();
            Some(parse_line(&line))
        })
        .flatten()
        .collect::<Vec<U256>>();

    println!("Parse took {}μs", start_time.elapsed().as_micros());

    let start_time = std::time::Instant::now();

    let part1_result = part1(&items, line_len);

    println!("part1 took {}μs", start_time.elapsed().as_micros());

    println!("part1: {}", part1_result);

    let start_time = std::time::Instant::now();

    let part2_result = part2(&items, line_len);

    println!("part2 took {}μs", start_time.elapsed().as_micros());

    println!("part2: {}", part2_result);

    Ok(())
}
