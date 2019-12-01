pub trait Challenge {
    fn run(&mut self, part: &str) -> ChallengeResult;
}

pub enum Err {
    MissingPart(String),
    Failure(String),
}

impl ToString for Err {
    fn to_string(&self) -> String {
        match self {
            Self::MissingPart(s) => s.clone(),
            Self::Failure(s) => s.clone(),
        }
    }
}

pub type ChallengeResult = Result<(), Err>;
