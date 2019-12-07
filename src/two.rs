use crate::challenge;
use std::error::Error;
use std::fmt;
use std::usize;

use crate::input;

pub struct Challenge {}

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

    fn one(&mut self) -> challenge::ChallengeResult {
        let script =
            input::string("2", "1.txt").map_err(|e| input::read_error("2", "1", "1.txt", e))?;

        let mut registers =
            Registers::from_program(&*script).map_err(|e| challenge::Err::Failure(e))?;

        registers.0[1] = 12;
        registers.0[2] = 2;

        match run_script(&mut registers) {
            Run::Finished => report_result(&registers),
            Run::Error(e) => Err(challenge::Err::Failure(e.description().to_owned())),
            _ => Err(challenge::Err::Failure(
                "Script did not run to completion".to_owned(),
            )),
        }
    }
}

fn run_script(registers: &mut Registers) -> Run {
    let mut state = EvalStep::new(0);
    loop {
        match run_step(&state, registers) {
            Run::Continue(next) => state = next,
            res => return res,
        }
    }
}

fn run_step(state: &EvalStep, registers: &mut Registers) -> Run {
    match state.eval(registers) {
        Err(e) => Run::Error(RunError(e.description().to_owned())),
        Ok((_, OpCode::Stop)) => Run::Finished,
        Ok((next, op)) => match op.apply(registers) {
            Err(e) => Run::Error(RunError(e.description().to_owned())),
            _ => Run::Continue(next),
        },
    }
}

fn report_result(regs: &Registers) -> challenge::ChallengeResult {
    regs.at(0)
        .map(|val| println!("Final value: {}", val))
        .map_err(|e| {
            challenge::Err::Failure(format!(
                "Cannot report result from invalid registers: {}",
                e
            ))
        })
}

enum Run {
    Finished,
    Continue(EvalStep),
    Error(RunError),
}

#[derive(Debug)]
struct RunError(String);

impl Error for RunError {}
impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error running script: {}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum OpCode {
    Stop,
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
}

impl OpCode {
    fn parse(position: usize, registers: &Registers) -> Result<Self, OpCodeParseError> {
        let code = registers
            .at(position)
            .map_err(|e| OpCodeParseError(e.description().to_owned()))?;

        match code {
            1 | 2 => registers
                .range(position + 1, position + 3)
                .map_err(|e| OpCodeParseError(e.description().to_owned()))
                .map(|nums| {
                    if *code == 1 {
                        Self::Add(nums[0], nums[1], nums[2])
                    } else {
                        Self::Mul(nums[0], nums[1], nums[2])
                    }
                }),
            99 => Ok(OpCode::Stop),
            c => Err(OpCodeParseError(format!(
                "Invalid op code {code:} at position {pos:}",
                code = c,
                pos = position
            ))),
        }
    }

    fn apply(&self, registers: &mut Registers) -> Result<(), OpApplicationError> {
        match self {
            Self::Stop => Err(OpApplicationError::Stop),
            Self::Add(xloc, yloc, resloc) => {
                let x = registers
                    .at(*xloc)
                    .map_err(|e| OpApplicationError::Arithmetic(e.description().to_owned()))?;

                let y = registers
                    .at(*yloc)
                    .map_err(|e| OpApplicationError::Arithmetic(e.description().to_owned()))?;

                let sum = x + y;

                registers
                    .set(*resloc, sum)
                    .map_err(|e| OpApplicationError::Arithmetic(e.description().to_owned()))
            }
            Self::Mul(xloc, yloc, resloc) => {
                let x = registers
                    .at(*xloc)
                    .map_err(|e| OpApplicationError::Arithmetic(e.description().to_owned()))?;
                let y = registers
                    .at(*yloc)
                    .map_err(|e| OpApplicationError::Arithmetic(e.description().to_owned()))?;

                let sum = x * y;

                registers
                    .set(*resloc, sum)
                    .map_err(|e| OpApplicationError::Arithmetic(e.description().to_owned()))
            }
        }
    }
}

#[derive(Debug)]
struct OpCodeParseError(String);

impl Error for OpCodeParseError {}
impl fmt::Display for OpCodeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failure parsing OpCode: {}", self.0)
    }
}

#[derive(Debug)]
enum OpApplicationError {
    Stop,
    Arithmetic(String),
}

impl Error for OpApplicationError {}
impl fmt::Display for OpApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Stop => write!(f, "Attempted to apply Stop op code"),
            Self::Arithmetic(msg) => write!(f, "Math failed: {}", msg),
        }
    }
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
        let len = self.0.len();
        if len < num {
            Err(RegisterErr::Insert(num))
        } else {
            self.0[num] = val;
            Ok(())
        }
    }

    fn range(&self, start: usize, end: usize) -> RegisterResult<&[usize]> {
        self.0
            .get(start..=end)
            .ok_or_else(|| RegisterErr::MissingRange(start, end))
    }
}

type RegisterResult<T> = Result<T, RegisterErr>;

#[derive(Debug)]
enum RegisterErr {
    Insert(usize),
    Missing(usize),
    MissingRange(usize, usize),
}

impl Error for RegisterErr {}
impl fmt::Display for RegisterErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Insert(p) => write!(f, "Failed to insert at position {}", p),
            Self::Missing(p) => write!(f, "No value at position {}", p),
            Self::MissingRange(s, e) => write!(f, "Value in range [{}, {}] is missing", s, e),
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

struct EvalStep {
    position: usize,
}

impl EvalStep {
    fn new(position: usize) -> Self {
        Self { position }
    }

    fn eval(&self, registers: &Registers) -> EvalResult {
        OpCode::parse(self.position, registers)
            .map_err(|e| EvalError(e.description().to_owned()))
            .map(|op| (Self::new(self.position + 4), op))
    }
}

type EvalResult = Result<(EvalStep, OpCode), EvalError>;

#[derive(Debug)]
struct EvalError(String);

impl Error for EvalError {}
impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error in evaluation: {}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let mut registers = Registers(vec![1, 0, 0, 0, 99]);
        run_script(&mut registers);
        assert_eq!(
            [2, 0, 0, 0, 99],
            *registers.range(0, 4).expect("test 1 failed")
        );

        let mut registers = Registers(vec![2, 3, 0, 3, 99]);
        run_script(&mut registers);
        assert_eq!(
            [2, 3, 0, 6, 99],
            *registers.range(0, 4).expect("test 2 failed")
        );

        let mut registers = Registers(vec![2, 4, 4, 5, 99, 0]);
        run_script(&mut registers);
        assert_eq!(
            [2, 4, 4, 5, 99, 9801],
            registers.range(0, 5).expect("test 3 failed")
        );

        let mut registers = Registers(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        run_script(&mut registers);
        assert_eq!(
            [30, 1, 1, 4, 2, 5, 6, 0, 99],
            registers.range(0, 8).expect("test 4 failed")
        );
    }

    #[test]
    fn test_op_parse() {
        let registers = Registers(vec![1, 0, 0, 0, 99]);
        let code = OpCode::parse(0, &registers).expect("Failed to parse add op code");
        assert_eq!(OpCode::Add(0, 0, 0), code);

        let registers = Registers(vec![2, 0, 0, 0, 99]);
        let code = OpCode::parse(0, &registers).expect("Failed to parse mul op code");
        assert_eq!(OpCode::Mul(0, 0, 0), code);

        let registers = Registers(vec![99, 0, 0, 0, 99]);
        let code = OpCode::parse(0, &registers).expect("Failed to parse stop op code");
        assert_eq!(OpCode::Stop, code);
    }

    #[test]
    fn test_op_apply() {
        let mut registers = Registers(vec![1, 0, 0, 0, 99]);
        let code = OpCode::Add(0, 1, 3);
        code.apply(&mut registers)
            .expect("Addition application failed");
        assert_eq!([1, 0, 0, 1, 99], registers.0[0..5]);

        let mut registers = Registers(vec![1, 3, 0, 0, 99]);
        let code = OpCode::Mul(0, 1, 3);
        code.apply(&mut registers)
            .expect("Multiplication application failed");
        assert_eq!([1, 3, 0, 3, 99], registers.0[0..5]);
    }
}
