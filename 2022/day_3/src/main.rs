use std::fs;
use std::collections::HashSet;
use itertools::Itertools;

fn get_common_2(l1: &str, l2: &str) -> char {
    let s1: HashSet<char> = l1.chars().collect();
    let s2: HashSet<char> = l2.chars().collect();
    *s1.intersection(&s2).next().expect("No intersection char found")
}

fn get_common_3(l1: &str, l2: &str, l3: &str) -> char {
    let s1: HashSet<char> = l1.chars().collect();
    let s2: HashSet<char> = l2.chars().collect();
    let s3: HashSet<char> = l3.chars().collect();
    let s4 = HashSet::from_iter(
        s1.intersection(&s2).into_iter().map(|ch| *ch)
    );

    *s4
        .intersection(&s3)
        .next()
        .expect("No intersection char found")
}

fn part1(input: &String) -> u32 {
    input.lines()
        .map(|line| get_common_2(&line[..line.len()/2], &line[line.len()/2..] ))
        .map(|ch|
            if ch.is_lowercase()
                {ch as u32 - 'a' as u32 + 1}
            else
                {ch as u32 - 'A' as u32 + 27}
        )
        .sum()
}

fn part2(input: &String) -> u32 {
    input.lines()
        .chunks(3)
        .into_iter()
        .map(|mut v|
            get_common_3(
                v.next().expect("d"),
                v.next().expect("d"),
                v.next().expect("d"))
        )
        .map(|ch|
            if ch.is_lowercase()
                {ch as u32 - 'a' as u32 + 1}
            else
                {ch as u32 - 'A' as u32 + 27}
        )
        .sum()
}

fn part1_alternative(input: &String) -> u32 {
    input.lines()
        .map(|line| (&line[..line.len()/2], &line[line.len()/2..]))
        .map(|(l1, l2 )|
            (l1.chars().collect::<HashSet<char>>(), l2.chars().collect::<HashSet<char>>())
        )
        .flat_map(|(hs1, hs2)| hs1.intersection(&hs2).cloned().collect::<Vec<char>>())
        .map(|c|
            match c {
                'a'..='z'=> c as u32 - 'a' as u32 + 1,
                'A'..='Z' => c as u32 - 'A' as u32 + 27,
                _ => 0
            }
        )
        .sum()
}

fn part2_alternative(input: &String) -> u32 {
    input.lines()
        .flat_map(|line| line.chars().collect::<HashSet<char>>())
        .chunks(3)
        .into_iter()
        .reduce(|e1, e1|, )
}

fn main() {
    let input = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", part1(&input));
    println!("Part 1: {}", part1_alternative(&input));
    println!("Part 2: {}", part2(&input));
}
