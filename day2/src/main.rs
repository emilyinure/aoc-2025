use std::{fs, ops::Range, str::Split};

fn is_valid_part1(num: &u64) -> bool {
    let str = num.to_string();
    if str.len() % 2 != 0 {
        return true;
    }

    let pattern = str.split_at(str.len() / 2).0;
    if pattern.repeat(2) == str {
        return false;
    }
    true
}

fn is_valid_part2(num: &u64) -> bool {
    let str = num.to_string();

    for i in 1..str.len() / 2 + 1 {
        let pattern = str.split_at(i).0;
        if pattern.repeat(str.len() / i) == str {
            return false;
        }
    }

    true
}

fn invalid_ids<P>(ids: Range<u64>, predicate: P) -> Vec<u64>
where
    P: Fn(&u64) -> bool,
{
    ids.filter(|num| !predicate(num)).collect()
}

fn str_split_to_range(mut split: Split<'_, char>) -> Option<Range<u64>> {
    Some(split.next()?.parse::<u64>().ok()?..split.next()?.parse::<u64>().ok()? + 1)
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let str_ranges = input.trim().split(',');

    let ranges: Vec<Range<u64>> = str_ranges
        .map(|str| str_split_to_range(str.split('-')))
        .flatten()
        .collect();

    let invalid_ids_part1 = ranges
        .clone()
        .into_iter()
        .map(|range| invalid_ids(range, is_valid_part1))
        .flatten()
        .collect::<Vec<u64>>();

    println!(
        "part1: {:?}",
        invalid_ids_part1.iter().fold(0, |acc, num| acc + num)
    );

    let invalid_ids_part2 = ranges
        .into_iter()
        .map(|range| invalid_ids(range, is_valid_part2))
        .flatten()
        .collect::<Vec<u64>>();

    println!(
        "part2: {:?}",
        invalid_ids_part2.iter().fold(0, |acc, num| acc + num)
    );

    Ok(())
}
