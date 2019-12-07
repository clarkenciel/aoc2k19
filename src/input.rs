use std::fs::File;
use std::io::{self, Read};

use crate::challenge;

pub fn file(challenge: &str, part: &str) -> io::Result<File> {
    File::open(format!(
        "inputs/{challenge:}/{part:}",
        challenge = challenge,
        part = part
    ))
}

pub fn string(challenge: &str, part: &str) -> io::Result<String> {
    file(challenge, part).and_then(|mut f| {
        let mut buf = String::new();
        f.read_to_string(&mut buf).map(|_| buf)
    })
}

pub fn read_error(challenge: &str, part: &str, filename: &str, e: io::Error) -> challenge::Err {
    challenge::Err::Failure(format!(
        "Failed to read input file {filename:} for part {part:} of challenge {challenge:}: {err:}",
        filename = filename,
        challenge = challenge,
        part = part,
        err = e.to_string()
    ))
}
