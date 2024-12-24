use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input_line_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let input:Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        let line = input.join("");

        let sum: usize = input_line_regex
            .captures_iter(&line)
            .map(|cap| {
                let left: usize = cap[1].parse().unwrap();
                let right: usize = cap[2].parse().unwrap();
                left * right
            })
            .sum();
        //println!("{}", safe_number);
        Ok(sum)
    }
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result:usize = time_snippet!({ part1(input_file)? });
    println!("Result = {}", result);

    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input_line_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let clean_regex = Regex::new(r"don't\(\).*?(do\(\)|$)").unwrap();

        let input:Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        let line = input.join("");
        let clean_line = clean_regex.replace_all(&line, "");

        let clean_sum: usize = input_line_regex
            .captures_iter(&clean_line)
            .map(|cap| {
                let left: usize = cap[1].parse().unwrap();
                let right: usize = cap[2].parse().unwrap();
                left * right
            })
            .sum();
        Ok(clean_sum)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
