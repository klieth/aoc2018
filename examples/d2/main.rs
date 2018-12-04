extern crate aoc2018;
extern crate itertools;

use aoc2018::run_tests;
use itertools::Itertools;

const TESTS_ONE: [(&'static str, usize); 1] = [
    (include_str!("test_one.txt"), 12),
];

//const TESTS_TWO: [(&'static str, &'static str); 1] = [
const TESTS_TWO: [(&'static str, &str); 1] = [
    (include_str!("test_two.txt"), "fgij"),
];

const INPUT: &'static str = include_str!("part_one.txt");

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

fn part_one(input: &str) -> usize {
    let (final_two, final_three) = input.lines().fold((0, 0), |(twos, threes), line| {
        let mut chars = line.chars().collect::<Vec<_>>();
        chars.sort();
        let first_group = chars.iter().group_by(|e| e.clone());
        let second_group = first_group.into_iter().filter_map(|(_, group)| {
            let count = group.count();
            if count == 2 || count == 3 {
                Some(count)
            } else {
                None
            }
        }).group_by(|e| e.clone());
        let map = second_group.into_iter().into_group_map();
        (
            twos   + if map.contains_key(&2) { 1 } else { 0 },
            threes + if map.contains_key(&3) { 1 } else { 0 }
        )
    });
    final_two * final_three
}

fn part_two<'a>(input: &str) -> String {
    input.lines().tuple_combinations().find_map(|(first, second)| {
        let count = first.len() - 1;
        let res : String = first.chars().zip(second.chars())
            .filter_map(|(f,s)| if f == s { Some(f) } else { None }).collect();
        if res.len() == count {
            Some(res)
        } else {
            None
        }
    }).expect("didn't find a match")
}
