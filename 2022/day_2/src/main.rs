use std::collections::HashMap;
use std::fs;

fn game_outcome(input: &str) -> u32 {
    let rock_paper_sciccors_score = HashMap::from([
        ("A Y", 6), //Rock
        ("A X", 3),
        ("A Z", 0),
        ("B Y", 3), //Paper
        ("B X", 0),
        ("B Z", 6),
        ("C Y", 0), //Scissors
        ("C X", 6),
        ("C Z", 3),
    ]);

    rock_paper_sciccors_score[input]
}

fn item_score(input: &str) -> u32 {
    let items_and_scores = HashMap::from([
        ("X", 1), //Rock
        ("Y", 2), //Paper
        ("Z", 3), //Scissors
    ]);

    items_and_scores[input.split(" ").last().expect("No last char")]
}

fn change_input(input: &str) -> &str {
    let rock_paper_sciccors_outcome = HashMap::from([
        ("A Y", "A X"), // draw
        ("A X", "A Z"), // lose
        ("A Z", "A Y"), // win
        ("B Y", "B Y"), // draw
        ("B X", "B X"), // lose
        ("B Z", "B Z"), // win
        ("C Y", "C Z"), // draw
        ("C X", "C Y"), // lose
        ("C Z", "C X"), // win
    ]);

    rock_paper_sciccors_outcome[input]
}

fn solve_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l|
            game_outcome(l) + item_score(l)
        ).sum()
}

fn solve_part_2(input: &str) -> u32 {
    input
    .lines()
    .map(|l|
        change_input(l)
    )
    .map(|l|
        game_outcome(l) + item_score(l)
    )
    .sum()
}

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    let test = contents
        .lines()
        .map(|s| s.split(" "))
        .map(|(a,b)| a)
        .collect();
        // .zip(|s| s.split(" "))
        // .collect();
        // .split(" ");
    println!("{:?}", test);

    // println!("Solution for part 1 {}", solve_part_1(&contents));
    // println!("Solution for part 1 {}", solve_part_2(&contents));
}
