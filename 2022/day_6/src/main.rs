use std::fs;
use std::collections::HashSet;

fn determine_unique(input: &str, unique: usize) -> bool {
    let hs = input.chars().collect::<HashSet<char>>();
    hs.len() == unique
}

fn part_1(input: &str) -> u32 {
    let mut start = 0;
    for i in 4..input.len()+1 {

        let test = &input[start..i];
        if determine_unique(test, 4) {
            return i as u32;
        }
        start += 1;
    }

    0
}

fn part_2(input: &str) -> u32 {
    let mut start = 0;
    for i in 14..input.len()+1 {

        let test = &input[start..i];
        if determine_unique(test, 14) {
            return i as u32;
        }
        start += 1;
    }

    0
}

fn main() {
    let input = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    println!("Answer for part 1: {}", part_1(&input));
    println!("Answer for part 2: {}", part_2(&input));
}
