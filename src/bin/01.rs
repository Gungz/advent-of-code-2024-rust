use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        // TODO: Solve Part 1 of the puzzle
        let input_line_regex = Regex::new(r"(\d+) {3}(\d+)").unwrap();
        let mut left_list:Vec<i32> = Vec::new();
        let mut right_list:Vec<i32> = Vec::new();

        let input:Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

        //println!("{:?}", input);

        for line in input {
            //println!("{}", line?);
            let captures = input_line_regex
                .captures(&line)
                .unwrap_or_else(|| panic!("Incorrect input line {}", line));

            left_list.push(captures[1].parse::<i32>()?);
            right_list.push(captures[2].parse::<i32>()?);
        }

        let mut left_sorted = left_list.clone();
        let mut right_sorted = right_list.clone();
        left_sorted.sort();
        right_sorted.sort();

        let differ: Vec<i32> = right_sorted.iter()
            .zip(left_sorted.iter())
            .map(|(el1, el2)| (el1 - el2).abs())
            .collect();

        let diff_total: i32 = differ.iter().sum();
        //println!("{}", diff_total);

        Ok(diff_total)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result:i32 = time_snippet!({ part1(input_file)? });
    println!("Result = {}", result);

    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let input_line_regex = Regex::new(r"(\d+) {3}(\d+)").unwrap();
        let mut left_list:Vec<i32> = Vec::new();
        let mut right_list:Vec<i32> = Vec::new();

        let input:Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

        //println!("{:?}", input);

        for line in input {
            //println!("{}", line?);
            let captures = input_line_regex
                .captures(&line)
                .unwrap_or_else(|| panic!("Incorrect input line {}", line));

            left_list.push(captures[1].parse::<i32>()?);
            right_list.push(captures[2].parse::<i32>()?);
        }

        let mut left_sorted = left_list.clone();
        let mut right_sorted = right_list.clone();
        left_sorted.sort();
        right_sorted.sort();

        let occurrence: Vec<i32> = left_list.iter()
            .map(|el| right_list.iter().filter(|&x| x == el).count() as i32)
            .collect();

        let similarities: Vec<i32> = left_list.iter()
            .zip(occurrence.iter())
            .map(|(el1, el2)| el1 * el2)
            .collect();

        let sim_total: i32 = similarities.iter().sum();
        Ok(sim_total)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
