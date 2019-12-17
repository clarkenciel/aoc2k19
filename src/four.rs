use std::convert::TryInto;
use std::iter;

use crate::challenge;
use crate::input;

pub struct Challenge {}

impl Challenge {
    pub fn new() -> Self {
        Self {}
    }
}

impl challenge::Challenge for Challenge {
    fn run(&mut self, _: &str) -> challenge::ChallengeResult {
        println!("{:?}", naive(134792, 675810));
        Ok(())
    }
}

fn naive(lo: u64, hi: u64) -> usize {
    (lo..=hi).filter(|n| is_valid(*n)).count()
}

fn is_valid(n: u64) -> bool {
    let digs = digits(n);
    digs.len() == 6 && n_contiguous(2, &digs) && ascending(&digs)
}

fn digits(n: u64) -> Vec<u64> {
    if (0..10).contains(&n) {
        return vec![n];
    }

    let mut digs = vec![];
    let mut shifted = n as f64;
    while shifted >= 1f64 {
        shifted = shifted / 10f64;
        let rem = shifted - shifted.trunc();
        println!("{:?}", (shifted, shifted.trunc()));
        let dig = rem * 10f64;
        println!("{:?}", dig);
        digs.push(dig as u64);
    }
    digs.iter().rev().cloned().collect()
}

fn n_contiguous(n: u64, ns: &[u64]) -> bool {
    ns.iter().zip(ns.iter().skip(1)).any(|(x, y)| x == y)
}

fn ascending(ns: &[u64]) -> bool {
    ns.iter().zip(ns.iter().skip(1)).all(|(x, y)| x <= y)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid() {
        assert!(is_valid(111111));
        assert!(!is_valid(223450));
        assert!(!is_valid(123789));
    }

    #[test]
    fn test_digits() {
        assert_eq!([1, 0, 0, 0], digits(1000)[..]);
        assert_eq!([1, 2, 3, 4], digits(1234)[..]);
        assert_eq!([0], digits(0)[..]);
        assert_eq!([1, 2, 3, 7, 8, 9], digits(123789)[..]);
    }

    #[test]
    fn test_ascending() {
        assert!(ascending(&[1, 2, 3, 4]));
        assert!(ascending(&[1, 2, 3, 3]));
        assert!(!ascending(&[1, 2, 3, 2]));
    }

    #[test]
    fn test_contiguous() {
        assert!(n_contiguous(2, &[1, 2, 2, 3, 4]));
        assert!(n_contiguous(2, &[1, 2, 2, 2, 3]));
        assert!(!n_contiguous(2, &[1, 2, 3, 4]));
    }
}
