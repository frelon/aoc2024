use std::env;
use std::fs;
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug,Clone)]
struct Report {
    levels: Vec<usize>,
}

impl FromStr for Report {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s.split_whitespace().map(|l| l.parse::<usize>().unwrap()).collect();

        Ok(Report{ levels })
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for l in &self.levels {
            write!(f, "{} ", l)?;
        }

        Ok(())
    }
}

#[derive(PartialEq)]
enum Mode {
    Increasing,
    Decreasing,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if *self == Mode::Increasing { "inc" } else {"dec"})?;
        Ok(())
    }
}

impl Report {
    fn is_safe(self:&Self) -> bool {
        let mode = if self.levels[0] < self.levels[self.levels.len()-1] { Mode::Increasing } else { Mode::Decreasing };

        for l in 0..self.levels.len()-1 {
            let diff = self.levels[l].abs_diff(self.levels[l+1]) as usize;

            if mode == Mode::Decreasing && self.levels[l] < self.levels[l+1]{
                return false
            }

            if mode == Mode::Increasing && self.levels[l] > self.levels[l+1]{
                return false
            }
            
            if diff < 1 || diff > 3 {
                return false
            }
        }

        true
    }
}

fn is_safe_damper(r:&Report) -> bool {
    if r.is_safe() {
        return true;
    }
    
    for n in 0..r.levels.len() {
        let mut clone = r.clone();
        clone.levels.remove(n);
        if clone.is_safe() {
            return true
        }
    }

    return false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let contents = fs::read_to_string(path)
        .expect(&format!("could not read file {}", path));

    let reports: Vec<Report> = contents.lines().map(|l| l.parse::<Report>().unwrap()).collect();

    println!("Part1: {}", part1(reports.clone()));
    println!("Part2: {}", part2(reports.clone()));
}

fn part1(reports: Vec<Report>) -> usize {
    reports.iter().filter(|&r| r.is_safe()).count()
}

fn part2(reports: Vec<Report>) -> usize {
    reports.iter().filter(|&r| is_safe_damper(r)).count()
}
