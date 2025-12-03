use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part1(batteries: &[u8]) -> Option<u8> {
    let mut highest = 0usize;
    let mut second_highest = 1usize;

    if batteries[second_highest] > batteries[highest] {
        std::mem::swap(&mut highest, &mut second_highest);
    }

    let (_, tens) = batteries.split_last()?;
    for i in (1..10).rev() {
        let index = tens.iter().position(|joltage| *joltage == i);

        if !index.is_some() {
            continue;
        }

        let index = index.unwrap();

        let remaining = batteries.split_at(index + 1).1;

        if let Some(ones) = remaining.iter().max() {
            return Some(batteries[index] * 10 + ones);
        }
    }
    None
}

fn part2(batteries: &[u8]) -> Option<u64> {
    let mut left_bound = 0usize;
    Some(
        (0..12)
            .rev()
            .map(|item_count| {
                let right_bound = batteries.len() - item_count;

                let window_slice = &batteries[left_bound..right_bound];

                let best = window_slice.iter().max()?;
                let rel_index = window_slice.iter().position(|v| v == best)?;

                left_bound += rel_index + 1usize;

                let mut best = *best as u64;

                best *= 10u64.pow(item_count as u32);

                Some(best)
            })
            .flatten()
            .sum(),
    )
}

fn main() -> anyhow::Result<()> {
    let input = File::open("input.txt")?;
    let input_reader = BufReader::new(input);

    let items = input_reader
        .lines()
        .map(|line| {
            Some(
                line.ok()?
                    .chars()
                    .flat_map(|char| Some(char.to_digit(10)? as u8))
                    .collect::<Vec<u8>>(),
            )
        })
        .flatten()
        .collect::<Vec<Vec<u8>>>();

    let part1_result: u64 = items
        .iter()
        .map(|item| Some(part1(item)? as u64))
        .flatten()
        .sum();

    println!("part1 {}", part1_result);

    let part2_result: u64 = items
        .iter()
        .map(|item| Some(part2(item)? as u64))
        .flatten()
        .sum();

    println!("part2 {}", part2_result);

    Ok(())
}
