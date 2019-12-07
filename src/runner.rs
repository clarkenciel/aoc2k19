use std::collections::HashMap;

use crate::challenge::Challenge;

use crate::one;
use crate::two;

pub struct Runner {
    challenges: HashMap<&'static str, Box<dyn Challenge>>,
}

impl Runner {
    pub fn new() -> Self {
        let mut challenges = HashMap::new();
        challenges.insert("one", Box::new(one::Challenge::new()) as Box<dyn Challenge>);
        challenges.insert("two", Box::new(two::Challenge::new()) as Box<dyn Challenge>);
        Self { challenges }
    }

    pub fn run(&mut self, day: &str, part: &str) -> Result<(), Err> {
        self.challenges
            .get_mut(day)
            .ok_or_else(|| {
                Err::MissingChallenge(format!("No challenge for day {:} has been registered", day))
            })?
            .run(part)
            .map_err(|e| Err::Failure(e.to_string()))
    }
}

pub enum Err {
    MissingChallenge(String),
    Failure(String),
}

impl ToString for Err {
    fn to_string(&self) -> String {
        match self {
            Self::MissingChallenge(s) => s.clone(),
            Self::Failure(s) => s.clone(),
        }
    }
}
