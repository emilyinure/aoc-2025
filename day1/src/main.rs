use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Rem,
};

fn dial(num: i32, rot: i32, clicks: &mut u32) -> i32 {
    *clicks += rot.unsigned_abs() / 100;

    let rem = (num as i32) + rot.rem(100);

    if num == 0 || rem < 0 || rem > 100 {
        *clicks += 1;
    }

    (num + rot).rem_euclid(100)
}

fn handle_input(mut num: u8, mut input: String, clicks: &mut u32) -> u8 {
    let rot = input.split_off(1);
    let rot = rot.parse::<i32>().unwrap_or(0);

    match input.chars().nth(0usize) {
        Some('R') => {
            num = dial(num as i32, rot, clicks).try_into().unwrap_or(num);
        }
        Some('L') => {
            num = dial(num as i32, -rot, clicks).try_into().unwrap_or(num);
        }
        Some(_) => {}
        None => {}
    }
    num
}

fn main() -> anyhow::Result<()> {
    let input = File::open("test.txt")?;
    let input_reader = BufReader::new(input);

    let mut clicks = 0;
    let mut zeros = 0;
    let mut num = 50;
    for line in input_reader.lines() {
        let line = line?;
        println!("{}", line.clone());
        num = handle_input(num, line, &mut clicks);
        if num == 0 {
            zeros += 1;
        }
    }

    println!("{}, {}", zeros, clicks);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_under() {
        let mut num = 1;
        let mut count = 0;
        handle_input(num, String::from("L101"), &mut count);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_over() {
        let mut num = 0;
        let mut count = 0;
        handle_input(num, String::from("R101"), &mut count);
        assert_eq!(count, 2);
    }
    #[test]
    fn test_over2() {
        let mut num = 99;
        let mut count = 0;
        handle_input(num, String::from("R1"), &mut count);
        assert_eq!(count, 0);
    }
}
