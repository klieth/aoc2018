extern crate aoc2018;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use aoc2018::run_tests;
use regex::Regex;

const TESTS_ONE : [(&str, usize); 1] = [
    (include_str!("tests_one.txt"), 4),
];
const TESTS_TWO : [(&str, usize); 1] = [
    (include_str!("tests_one.txt"), 3),
];

const INPUT : &str = include_str!("part_one.txt");

fn main() {
    if !run_tests(&TESTS_ONE, part_one) {
        println!("Tests failed, exiting");
        return;
    }
    println!("PART ONE: {}", part_one(INPUT));

    if !run_tests(&TESTS_TWO, part_two) {
        println!("Tests failed, exiting");
        return;
    }
    println!("PART TWO: {}", part_two(INPUT));
}

fn part_two(input: &str) -> usize {
    lazy_static! { static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap(); }

    let mut board = std::collections::HashMap::new();
    let mut candidates = std::collections::HashSet::new();

    for line in input.lines() {
        let caps = RE.captures(line).expect(&format!("failed on: {}", line));
        let id = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let x = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let y = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let w = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let h = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();

        let mut is_candidate = true;
        for i in 0..w {
            for j in 0..h {
                let entry = board.entry( (x + i, y + j) ).or_insert(vec!());
                if entry.len() != 0 {
                    is_candidate = false;
                    entry.iter().for_each(|i| {candidates.remove(i);});
                }
                entry.push(id);
            }
        }
        if is_candidate {
            candidates.insert(id);
        }
    }
    if candidates.len() != 1 { panic!("too many candidates: {:?}", candidates) }
    *candidates.iter().next().unwrap()
}

fn part_one(input: &str) -> usize {
    lazy_static! { static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap(); }

    let mut board = std::collections::HashMap::new();

    for line in input.lines() {
        let caps = RE.captures(line).expect(&format!("failed on: {}", line));
        //let id = caps.get(1).parse().unwrap();
        let x = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let y = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let w = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let h = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();

        for i in 0..w {
            for j in 0..h {
                *board.entry( (x + i, y + j) ).or_insert(0) += 1;
            }
        }
    }
    board.values().fold(0, |acc, v| if *v > 1 { acc + 1 } else { acc })
}
