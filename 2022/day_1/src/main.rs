use std::fs;

fn solve_part_1(input: &String) -> u32 {
    input
        .split("\n\n")
        .map(|c|
            c.lines()
            .map(|s| s.parse::<u32>().expect("NaN"))
                .sum()
        )
        .max()
        .expect("Not working")
}

fn solve_part_2(input: &String) -> u32 {
    let mut callories: Vec<u32> = input
        .split("\n\n")
        .map(|c|
            c.lines()
            .map(|s| s.parse::<u32>().expect("NaN"))
                .sum()
        ).collect();

    callories.sort();
    callories.iter().rev().take(3).sum()
}

fn main() {
    let contents = fs::read_to_string("./src/input1.txt")
        .expect("Should have been able to read the file");

    println!("part 1 solution: {}", solve_part_1(&contents));
    println!("part 2 solution: {}", solve_part_2(&contents));
}
