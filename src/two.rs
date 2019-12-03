use crate::challenge;
use std::error::Error;
use std::fmt;
use std::usize;

struct Challenge {}

impl challenge::Challenge for Challenge {
    fn run(&mut self, part: &str) -> challenge::ChallengeResult {
        match part {
            "one" => self.one(),
            _ => Err(challenge::Err::MissingPart(format!(
                "No part {part:} available",
                part = part
            ))),
        }
    }
}

impl Challenge {
    pub fn new() -> Self {
        Self {}
    }

    fn one(&mut self) -> challenge::ChallengeResult {}
}

enum OpCode {
    Stop,
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
}

struct Registers(Vec<usize>);

impl Registers {
    fn from_program(program: &str) -> Result<Self, String> {
        let mut regs = vec![];
        let result: Result<(), ProgramParseError> =
            program.lines().enumerate().try_for_each(|(line_no, line)| {
                line.split(',')
                    .enumerate()
                    .map(|(col_no, num_string)| {
                        usize::from_str_radix(num_string, 10).map_err(|_| ProgramParseError {
                            line_no,
                            col_no,
                            bad_val: num_string.to_owned(),
                        })
                    })
                    .collect::<Result<Vec<usize>, ProgramParseError>>()
                    .map(|nums| {
                        for num in nums {
                            regs.push(num);
                        }
                    })
            });

        match result {
            Err(e) => Err(e.description().to_string()),
            _ => Ok(Self(regs)),
        }
    }

    fn at(&self, num: usize) -> RegisterResult<&usize> {
        self.0.get(num).ok_or_else(|| RegisterErr::Missing(num))
    }

    fn set(&mut self, num: usize, val: usize) -> RegisterResult<()> {
        if self.0.len() < num {
            Err(RegisterErr::Insert(num))
        } else {
            self.0[num] = val;
            Ok(())
        }
    }
}

type RegisterResult<T> = Result<T, RegisterErr>;

#[derive(Debug)]
enum RegisterErr {
    Insert(usize),
    Missing(usize),
}

impl Error for RegisterErr {}
impl fmt::Display for RegisterErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Insert(p) => write!(f, "Failed to insert at position {}", p),
            Self::Missing(p) => write!(f, "No value at position {}", p),
        }
    }
}

#[derive(Debug)]
struct ProgramParseError {
    line_no: usize,
    col_no: usize,
    bad_val: String,
}

impl Error for ProgramParseError {}
impl fmt::Display for ProgramParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Parsing failure at {},{}: {}",
            self.line_no, self.col_no, self.bad_val
        )
    }
}

struct Evaluator {
    position: usize,
    registers: Registers,
}

impl Evaluator {
    fn new(registers: Registers) -> Self {
        Self {
            position: 0,
            registers,
        }
    }

    fn from_err(registers: Registers, err: SystemError) -> Result<Self, String> {
        match err {
            SystemError::Alarm(v1, v2) => {
                if let Err(e) = registers.set(1, v1) {
                    return Err(e.description().to_owned());
                }

                if let Err(e) = registers.set(2, v2) {
                    return Err(e.description().to_owned());
                }

                Ok(Self::new(registers))
            }
        }
    }

    fn evaluate(&mut self, code: &OpCode) -> EvalResult {
        match code {
            OpCode::Stop => Ok(EvalSignal::Finished),
            OpCode::Add(x, y, result) => match (self.registers.at(*x), self.registers.at(*y)) {
                (Ok(x_val), Ok(y_val)) => match self.registers.set(*result, x_val + y_val) {
                    Err(e) => Err(EvalErr::AddErr(Box::new(e))),
                    _ => {
                        self.position += 4;
                        Ok(EvalSignal::Continue)
                    }
                },
                (Err(e), _) => Err(EvalErr::AddErr(Box::new(e))),
                (_, Err(e)) => Err(EvalErr::AddErr(Box::new(e))),
            },
            OpCode::Mul(x, y, result) => match (self.registers.at(*x), self.registers.at(*y)) {
                (Ok(x_val), Ok(y_val)) => match self.registers.set(*result, x_val * y_val) {
                    Err(e) => Err(EvalErr::MulErr(Box::new(e))),
                    _ => {
                        self.position += 4;
                        Ok(EvalSignal::Continue)
                    }
                },
                (Err(e), _) => Err(EvalErr::MulErr(Box::new(e))),
                (_, Err(e)) => Err(EvalErr::MulErr(Box::new(e))),
            },
        }
    }
}

type EvalResult = Result<EvalSignal, EvalErr>;

enum EvalSignal {
    Continue,
    Finished,
}

enum EvalErr {
    AddErr(Box<dyn Error>),
    MulErr(Box<dyn Error>),
}

enum SystemError {
    Alarm(usize, usize),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_evaluator() {
        let ev = Evaluator::new(Registers::from_program("1,0,0,0,99").unwrap());
        let ev = Evaluator::new(Registers::from_program("2,3,0,3,9").unwrap());
        let ev = Evaluator::new(Registers::from_program("2,4,4,5,99,0").unwrap());
        let ev = Evaluator::new(Registers::from_program("1,1,1,4,99,5,6,0,99").unwrap());
    }
}
