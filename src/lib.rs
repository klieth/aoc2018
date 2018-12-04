use std::cmp::PartialEq;
use std::fmt::Display;

pub fn run_tests<I: Clone, O, J: PartialEq<O> + Display, F: Fn(I) -> J>(tests: &[(I, O)], runner: F) -> bool {
    for (index, (input, output)) in tests.iter().enumerate() {
        println!("TEST {}", index + 1);
        let result = runner(input.clone());
        if result == *output {
            println!("  PASS: {}", result);
        } else {
            println!("  FAIL: {}", result);
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
