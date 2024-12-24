use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

#[derive(PartialEq)]
enum Increment {
    Up,
    Down,
}

fn check_is_report_safe(numbers: &[&str]) -> bool {
    let mut is_safe = true;
    let mut increment = None;
    let mut prev_number = i32::MIN;
    let mut curr_number = i32::MAX;

    for (i, num) in numbers.iter().enumerate() {
        if i == 0 {
            curr_number = num.parse().unwrap();
        } else {
            prev_number = numbers[i-1].parse().unwrap();
            curr_number = num.parse().unwrap();
            let diff = curr_number - prev_number;

            if diff > 0 && diff <= 3 {
                if increment.is_none() {
                    increment = Some(Increment::Up);
                } else if increment == Some(Increment::Down) {
                    is_safe = false;
                    break;
                }
            } else if diff < 0 && diff >= -3 {
                if increment.is_none() {
                    increment = Some(Increment::Down);
                } else if increment == Some(Increment::Up) {
                    is_safe = false;
                    break;
                }
            } else {
                is_safe = false;
                break;
            }
        }
    }
    is_safe
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input:Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        let mut results = Vec::new();
        for line in input {
            let numbers: Vec<&str> = line.split_whitespace().collect();
            results.push(check_is_report_safe(&numbers));
        }
        let safe_number = results.iter().filter(|&&x| x).count();
        //println!("{}", safe_number);
        Ok(safe_number)
    }
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result:usize = time_snippet!({ part1(input_file)? });
    println!("Result = {}", result);

    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input:Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        let mut results = Vec::new();
        for line in &input {
            let numbers: Vec<&str> = line.split_whitespace().collect();
            results.push(check_is_report_safe(&numbers));
        }
        for x in 0..results.len() {
            if !results[x] {
                let numbers: Vec<&str> = input[x].split_whitespace().collect();
                for i in 0..numbers.len() {
                    let mut new_numbers = numbers.clone();
                    new_numbers.remove(i);
                    if check_is_report_safe(&new_numbers) {
                        results[x] = true;
                        break;
                    }
                }
            }
        }

        let dampened_safe_number = results.iter().filter(|&&x| x).count();
        Ok(dampened_safe_number)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
