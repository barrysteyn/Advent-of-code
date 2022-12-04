use std::fs;
use core::ops::Range;

fn convert_string_to_range(input: &str) -> Range<u32> {
    let r: Vec<u32> =
    input.split("-")
        .map(|spl|
            spl.parse::<u32>().expect("Error Parsing")
        )
        .collect();

    r[0]..r[1]
}

fn convert_input(input: &String) -> Vec<(Range<u32>,Range<u32>)> {
    input.lines()
        .map(|line| line.split(","))
        .map(|mut   spl|
            (
                spl.next().expect("Error"),
                spl.next().expect("Error")
            )
        )
        .map(|(a,b)|
            (convert_string_to_range(a),convert_string_to_range(b)))
        .collect()
}

fn part_1(input: &String) -> usize {
    convert_input(input)
        .iter()
        .map(|(r1, r2)|
            r1.start <= r2.start && r2.end <= r1.end ||
            r2.start <= r1.start && r1.end <= r2.end
        )
        .filter(|b| *b == true)
        .count()
}

fn part_2(input: &String) -> usize {
    convert_input(input)
        .iter()
        .map(|(r1, r2)|
            r1.start <= r2.start && r1.end >= r2.start ||
            r2.start <= r1.start && r2.end >= r1.start
        )
        .filter(|b| *b == true)
        .count()
}

fn main() {
    let input = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    println!("Solution to part1 {}", part_1(&input));
    println!("Solution to part2 {}", part_2(&input));
}
