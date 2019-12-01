use std::fs::File;
use std::io::{self, Read};
use std::u64;

use crate::challenge;

pub struct Challenge {}

impl challenge::Challenge for Challenge {
    fn run(&mut self, part: &str) -> challenge::ChallengeResult {
        match part {
            "one" => self.part_one(),
            "two" => self.part_two(),
            _ => Err(challenge::Err::MissingPart(part.to_owned())),
        }
    }
}

impl Challenge {
    pub fn new() -> Self {
        Self {}
    }

    fn part_one(&mut self) -> challenge::ChallengeResult {
        input("1.txt")
            .map_err(|e| file_read_error("one", e))
            .and_then(|s| calculate_fuel(&mut s.lines(), fuel_requirement))
            .map(|total| println!("{}", total))
    }

    fn part_two(&mut self) -> challenge::ChallengeResult {
        input("2.txt")
            .map_err(|e| file_read_error("two", e))
            .and_then(|s| calculate_fuel(&mut s.lines(), recursive_fuel_requirement))
            .map(|total| println!("{}", total))
    }
}

fn fuel_requirement(mass: u64) -> u64 {
    (mass / 3).checked_sub(2).unwrap_or(0)
}

fn recursive_fuel_requirement(mass: u64) -> u64 {
    let mut total = 0;
    let mut adjustment = fuel_requirement(mass);
    while adjustment > 0 {
        total += adjustment;
        adjustment = fuel_requirement(adjustment);
    }
    total
}

fn calculate_fuel<'a, I: Iterator<Item = &'a str>, F: Fn(u64) -> u64>(
    modules: &mut I,
    fuel_requirement: F,
) -> Result<u64, challenge::Err> {
    modules.try_fold(0, |sum, line| {
        u64::from_str_radix(line, 10)
            .map(|n| fuel_requirement(n) + sum)
            .map_err(|e| {
                challenge::Err::Failure(format!(
                    "Failed to parse line containing {num:}: {msg:}",
                    num = line,
                    msg = e.to_string(),
                ))
            })
    })
}

fn input_file(part: &str) -> io::Result<File> {
    File::open(format!("inputs/1/{part:}", part = part))
}

fn input(part: &str) -> io::Result<String> {
    input_file(part).and_then(|mut f| {
        let mut buf = String::new();
        f.read_to_string(&mut buf).map(|_| buf)
    })
}

fn file_read_error(filename: &str, e: io::Error) -> challenge::Err {
    challenge::Err::Failure(format!(
        "Failed to read input file for part {part:} of challenge one: {err:}",
        part = filename,
        err = e.to_string()
    ))
}

#[cfg(test)]
mod test {
    #[test]
    fn test_fuel_requirement() {
        use super::fuel_requirement;

        assert_eq!(fuel_requirement(12), 2);
        assert_eq!(fuel_requirement(14), 2);
        assert_eq!(fuel_requirement(1969), 654);
        assert_eq!(fuel_requirement(100756), 33583);
    }

    #[test]
    fn test_recursive_fuel_requirement() {
        use super::recursive_fuel_requirement;

        assert_eq!(recursive_fuel_requirement(14), 2);
        assert_eq!(recursive_fuel_requirement(1969), 966);
        assert_eq!(recursive_fuel_requirement(100756), 50346);
    }
}
