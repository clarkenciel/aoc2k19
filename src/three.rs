use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::hash::Hash;
use std::iter;
use std::str::FromStr;

use crate::challenge;
use crate::input;

pub struct Challenge {}

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
    pub fn new() -> Self {
        Self {}
    }

    fn run_one(&self) -> challenge::ChallengeResult {
        input::string("3", "1.txt")
            .map_err(|e| input::read_error("3", "1", "1.txt", e))
            .map(|s| println!("{}", calculate(&*s)))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Motion(Direction, u64);

impl Motion {
    fn expand(&self) -> impl Iterator<Item = Self> {
        iter::repeat(Self(self.0, 1)).take(self.1 as usize)
    }
}

impl FromStr for Motion {
    type Err = ParseMotionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, rest) = s.split_at(1);

        str::parse::<Direction>(dir)
            .map_err(|e| ParseMotionError(s.to_owned(), Box::new(e)))
            .and_then(|d| {
                str::parse::<u64>(rest)
                    .map_err(|e| ParseMotionError(s.to_owned(), Box::new(e)))
                    .map(|n| Motion(d, n))
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
struct Pos(i64, i64);

impl Pos {
    fn origin() -> Self {
        Self(0, 0)
    }

    fn mv(&self, mot: &Motion) -> Self {
        match mot.0 {
            Direction::U => Self(self.0, self.1 + mot.1 as i64),
            Direction::R => Self(self.0 + mot.1 as i64, self.1),
            Direction::D => Self(self.0, self.1 - mot.1 as i64),
            Direction::L => Self(self.0 - mot.1 as i64, self.1),
        }
    }

    fn dist(&self, other: &Self) -> u64 {
        ((other.0 - self.0).abs() + (other.1 - self.1).abs()) as u64
    }
}

fn calculate(input: &str) -> u64 {
    let parsed_wires = input
        .lines()
        .map(|s| s.split(",").map(|ms| ms.parse()).collect())
        .collect::<Result<Vec<Vec<Motion>>, ParseMotionError>>();

    match parsed_wires {
        Err(e) => panic!("{}", e.description()),
        Ok(mots) => {
            let mots = mots.get(0..=1).expect("No instruction streams");

            let one = positions(&mots[0]);
            let two = positions(&mots[1]);

            let zero = Pos::origin();
            let intersect = one.intersection(&two);
            intersect
                .into_iter()
                .map(|p| p.dist(&zero))
                .min()
                .expect("no minimum")
        }
    }
}

fn positions(motions: &Vec<Motion>) -> HashSet<Pos> {
    motions
        .iter()
        .flat_map(Motion::expand)
        .scan(Pos::origin(), |p, m| {
            *p = p.mv(&m);
            Some(*p)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate() {
        let input_str = "R8,U5,L5,D3
U7,R6,D4,L4";
        let expectation = 6;
        let result = calculate(input_str);
        assert_eq!(expectation, result);

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

    #[test]
    fn test_motion_expand() {
        let mot = Motion(Direction::U, 10);
        let mots: Vec<Motion> = mot.expand().collect();
        assert_eq!(
            [
                Motion(Direction::U, 1),
                Motion(Direction::U, 1),
                Motion(Direction::U, 1),
                Motion(Direction::U, 1),
                Motion(Direction::U, 1),
                Motion(Direction::U, 1),
                Motion(Direction::U, 1),
                Motion(Direction::U, 1),
                Motion(Direction::U, 1),
                Motion(Direction::U, 1),
            ],
            mots[..]
        );
    }
}
