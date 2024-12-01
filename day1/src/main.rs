use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[2];

    let contents = fs::read_to_string(path)
        .expect(&format!("could not read file {}", path));

    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    for line in contents.lines() {
        let mut split = line.split_whitespace();

        let l = split.next().unwrap().parse::<usize>().unwrap();
        left.push(l);

        let r = split.next().unwrap().parse::<usize>().unwrap();
        right.push(r);
    }

    println!("Part 1: {}", part1(left.clone(), right.clone()));
    println!("Part 2: {}", part2(left.clone(), right.clone()));
}

fn part1(mut left:Vec<usize>,mut right:Vec<usize>) -> usize {
    left.sort();
    right.sort();

    let mut answer = 0;
    for n in 0..left.len() {
        let diff = left[n].abs_diff(right[n]) as usize;
        answer += diff;
    }

    answer
}

fn part2(left: Vec<usize>,right:Vec<usize>) -> usize {
    let mut map : HashMap<usize, usize>= HashMap::new();

    for num in right {
        *map.entry(num).or_insert(0) += 1;
    }

    let mut answer = 0;
    for n in 0..left.len() {
        let mul = match map.get(&left[n]) {
            Some(count) => *count,
            None => 0 as usize,
        };

        let similarity = left[n] * mul;
        answer += similarity;
    }

    answer
}
