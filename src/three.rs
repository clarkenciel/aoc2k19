use std::error::Error;
use std::fmt;
use std::ops::Add;
use std::str::FromStr;

use crate::challenge;

struct Challenge {}

impl challenge::Challenge for Challenge {
    fn run(&mut self, part: &str) -> challenge::ChallengeResult {
        match part {
            "one" => self.run_one(),
            p => Err(challenge::Err::MissingPart(format!(
                "Part {} is not implemented",
                p
            ))),
        }
    }
}

impl Challenge {
    fn new() -> Self {
        Self {}
    }

    fn run_one(&self) -> challenge::ChallengeResult {
        Ok(())
    }
}

#[derive(Debug)]
enum Direction {
    U,
    R,
    D,
    L,
}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::U),
            "R" => Ok(Self::R),
            "D" => Ok(Self::D),
            "L" => Ok(Self::L),
            _ => Err(ParseDirectionError(s.to_owned())),
        }
    }
}

#[derive(Debug)]
struct ParseDirectionError(String);

impl Error for ParseDirectionError {}
impl fmt::Display for ParseDirectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is not a valid direction", self.0)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Motion(i64, i64);

impl Add for &Motion {
    type Output = Motion;

    fn add(self, other: Self) -> Self::Output {
        Motion(self.0 + other.0, self.1 + other.1)
    }
}

impl FromStr for Motion {
    type Err = ParseMotionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, rest) = s.split_at(1);

        str::parse::<Direction>(dir)
            .map_err(|e| ParseMotionError(s.to_owned(), Box::new(e)))
            .and_then(|d| {
                str::parse::<i64>(rest)
                    .map_err(|e| ParseMotionError(s.to_owned(), Box::new(e)))
                    .and_then(|n| match d {
                        Direction::U => Ok(Motion(n, 0)),
                        Direction::R => Ok(Motion(0, n)),
                        Direction::D => Ok(Motion(-n, 0)),
                        Direction::L => Ok(Motion(0, -n)),
                    })
            })
    }
}

#[derive(Debug)]
struct ParseMotionError(String, Box<dyn Error>);

impl Error for ParseMotionError {}
impl fmt::Display for ParseMotionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Motion parse failed on string \"{string:}\": {cause:}",
            string = self.0,
            cause = self.1
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
struct Pos(i64, i64);

impl Pos {
    fn origin() -> Self {
        Self(0, 0)
    }

    fn mv(&self, mot: &Motion) -> Self {
        Pos(self.0 + mot.0, self.1 + mot.1)
    }

    fn dist(&self, other: &Self) -> u64 {
        ((other.0 - self.0) + (other.1 + self.1)).abs() as u64
    }
}

fn calculate(input: &str) -> u64 {
    let parsed_wires = input
        .lines()
        .map(|s| s.split(",").map(|ms| str::parse(ms)).collect())
        .collect::<Result<Vec<Vec<Motion>>, ParseMotionError>>();
    match parsed_wires {
        Err(e) => panic!("{}", e.description()),
        Ok(mots) => {
            let [one, two] = mots.get(0..=1).unwrap();
            let zero = Pos::origin();

            one.iter()
                .zip(two.iter())
                .scan((Pos::origin(), Pos::origin()), |(p1, p2), (m1, m2)| {
                    Some((p1.mv(m1), p2.mv(m2)))
                })
                .filter_map(|(p1, p2)| if p1 == p1 { Some(p1) } else { None })
                .min_by_key(|p| zero.dist(&p))
                .unwrap()
                .dist(&zero)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input_str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        let expectation = 159;
        let result = calculate(input_str);
        assert_eq!(expectation, result);

        let input_str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let expectation = 135;
        let result = calculate(input_str);
        assert_eq!(expectation, result);
    }
}
