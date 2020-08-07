use crate::dice::roll;
use lazy_static::lazy_static;
use regex::Regex;


pub fn parse_roll(s: &str) -> u32 {
    let mut total = 0;
    lazy_static! {
        static ref DIE_REGEX: Regex = Regex::new(
            r"(\d+)d(\d+)(\+\d+)?"
        ).unwrap();
    }
    for caps in DIE_REGEX.captures_iter(s) {
        let times: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let n: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let modifier: u32 = {
            match caps.get(3) {
                Some(num) => num.as_str().parse().unwrap(),
                None => 0
            }
        };
        total += roll(n, times) + modifier;
    }
    total
}
