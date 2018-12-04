#![feature(cell_update)]

const TESTS_1 : [(&'static str, isize); 4] = [
    ("+1\n-2\n+3\n+1", 3),
    ("+1\n+1\n+1", 3),
    ("+1\n+1\n-2", 0),
    ("-1\n-2\n-3", -6),
];
const TESTS_2 : [(&'static str, isize); 4] = [
    ("+1\n-1", 0),
    ("+3\n+3\n+4\n-2\n-4", 10),
    ("-6\n+3\n+8\n+5\n-6", 5),
    ("+7\n+7\n-2\n-7\n-4", 14),
];

const INPUT : &'static str = include_str!("./input.txt");

fn main() {
    println!("PART 1");
    for (index, (input, output)) in TESTS_1.iter().enumerate() {
        println!("Test {}:", index);
        let result = part_one(input);
        if result == *output {
            println!("PASS: {}", result);
        } else {
            println!("FAIL: {}", result);
            return;
        }
    }

    println!("ANSWER: {}", part_one(INPUT));
    println!();

    println!("PART 2");
    for (index, (input, output)) in TESTS_2.iter().enumerate() {
        println!("Test {}:", index);
        let result = part_two(input);
        if result == *output {
            println!("PASS: {}", result);
        } else {
            println!("FAIL: {}", result);
            return;
        }
    }

    println!("ANSWER: {}", part_two(INPUT));
}

fn part_one(input: &'static str) -> isize {
    input.lines().map(|l| l.parse::<isize>().unwrap()).sum()
}

/*
fn part_two(input: &'static str) -> isize {
    let mut set = std::collections::HashSet::new();
    let frequency = std::cell::Cell::new(0);

    input.lines().map(|l| l.parse::<isize>().unwrap()).cycle()
        .take_while(|_| set.insert(frequency.get()))
        .for_each(|n| { frequency.update(|old| old + n); });

    frequency.get()
}
*/

fn part_two(input: &'static str) -> isize {
    let input : Vec<isize> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut frequencies = std::collections::HashSet::new();
    let mut current = 0;
    frequencies.insert(current);
    for num in input.iter().cycle() {
        current = current + num;
        if !frequencies.insert(current) {
            return current;
        }
    }
    unreachable!();
}
