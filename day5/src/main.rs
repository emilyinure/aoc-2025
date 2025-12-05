use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
};

fn part1(available_ids: &Vec<u64>, fresh_ids: &Vec<Range<u64>>) -> u64 {
    available_ids
        .into_iter()
        .filter(|id| fresh_ids.iter().any(|range| range.contains(id)))
        .count() as u64
}

fn part2(mut fresh_ids: Vec<Range<u64>>) -> u64 {
    fresh_ids.sort_unstable_by_key(|range| range.start);

    fresh_ids
        .into_iter()
        .fold((0, u64::max_value()), |(count, last_end), range| {
            if last_end < range.start || last_end == u64::max_value() {
                (count + (range.end - range.start), range.end)
            } else if last_end >= range.start && last_end <= range.end {
                (count + (range.end - last_end).max(0), range.end)
            } else {
                (count, last_end)
            }
        })
        .0
}

fn main() -> anyhow::Result<()> {
    let input = File::open("input.txt")?;
    let input_reader = BufReader::new(input);

    let mut fresh_ids: Vec<Range<u64>> = Vec::new();
    let mut available_ids = Vec::new();
    input_reader.lines().try_for_each(|line| {
        let line = line.ok()?;

        if let Some(split) = line.split_once("-") {
            fresh_ids.push(split.0.parse().ok()?..split.1.parse::<u64>().ok()? + 1);
        } else if let Some(available) = line.parse::<u64>().ok() {
            available_ids.push(available);
        }
        Some(())
    });

    println!("hi");
    println!("{}", part1(&available_ids, &fresh_ids));
    println!("{}", part2(fresh_ids));

    println!("Hello, world!");
    Ok(())
}
