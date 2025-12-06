use std::{
    fs::File,
    io::{BufRead, BufReader},
};

enum Sign {
    Add,
    Multiply,
}

fn part1(data: &[String]) -> Option<u64> {
    let data = data
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
    let (last, data) = data.split_last()?;

    let comparisons: Vec<Sign> = last
        .into_iter()
        .map(|sign| match sign.as_str() {
            "+" => Some(Sign::Add),
            "*" => Some(Sign::Multiply),
            _ => None,
        })
        .flatten()
        .collect();

    let data = data
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|value| Some(value.parse::<u64>().ok()?))
                .flatten()
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let (initial_value, data) = data.split_first()?;

    Some(
        data.iter()
            .fold(initial_value.clone(), |mut results, row| {
                for (index, sign) in comparisons.iter().enumerate() {
                    match sign {
                        Sign::Add => {
                            results[index] += row[index];
                        }
                        Sign::Multiply => {
                            results[index] *= row[index];
                        }
                    };
                }
                results
            })
            .iter()
            .sum(),
    )
}

fn part2(data: &[String]) -> Option<u64> {
    let (last, data) = data.split_last()?;

    let comparisons: Vec<Sign> = last
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .into_iter()
        .map(|sign| match sign.as_str() {
            "+" => Some(Sign::Add),
            "*" => Some(Sign::Multiply),
            _ => None,
        })
        .flatten()
        .collect();

    let line_width = data[0].len(); // assuming they are all the same length

    let data: Vec<Vec<u64>> = (0usize..line_width)
        .map(|x| {
            data.iter()
                .flat_map(|row| row.chars().nth(x))
                .collect::<String>()
        })
        .fold(vec![Vec::new()], |mut rows, value| {
            let value = value
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse::<u64>();
            if let Ok(value) = value
                && let Some(last) = rows.last_mut()
            {
                last.push(value);
            } else {
                rows.push(vec![]);
            }
            rows
        });

    Some(
        comparisons
            .iter()
            .enumerate()
            .flat_map(|(index, comparison)| match comparison {
                Sign::Add => Some(data[index].iter().sum::<u64>()),
                Sign::Multiply => {
                    let mut iter = data[index].iter();
                    let initial = iter.next()?;
                    Some(iter.fold(*initial, |out, val| out * val))
                }
            })
            .sum(),
    )
}

fn main() -> anyhow::Result<()> {
    let input = File::open("input.txt")?;
    let input_reader = BufReader::new(input);

    let data = input_reader.lines().flatten().collect::<Vec<String>>();

    println!("{}", part1(&data).expect("err"));
    println!("{}", part2(&data).expect("err"));

    Ok(())
}
