use std::fs;

fn parse_lines_to_tuple(input: &str) -> Vec<(usize, char)> {
    input.lines()
        .flat_map(|l|
            l
                .chars()
                .enumerate()
                .filter(|(_, ch)|
                    match ch {
                        'A'..='Z' => true,
                        _ => false,
                    }
                ).collect::<Vec<_>>()
        )
        .collect()
}

fn make_stacks(input: Vec<(usize, char)>) -> Vec<Vec<char>> {
    let mut v: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        v.push(Vec::new());
    }

    for (i, ch) in input {
        let index = i/4 + i%4 - 1;
        v[index].push(ch);
    }

    v.into_iter()
        .map(|v| v.into_iter().rev().collect())
        .collect()
}

fn instructions(input: &str) -> Vec<(u32, u32, u32)> {
    input.lines()
    .map(|l| {
        let parts: Vec<&str> = l
            .splitn(6, ' ')
            .collect();
        let count = parts[1].parse().expect("bad count");
        let from = parts[3].parse().expect("bad from");
        let to = parts[5].parse().expect("bad to");
        (count, from, to)
    })
    .collect()
}

fn part_1(stacks: &mut Vec<Vec<char>>, instructions: Vec<(u32, u32, u32)>) -> String {
    for (m ,f ,t) in instructions {
        for _ in 0..m {
            let st_from = stacks.get_mut(f as usize-1).expect("Bad stack");
            let c = st_from.pop().expect("Nothing to get");
            let st_to = stacks.get_mut(t as usize-1).expect("Bad stack");
            st_to.push(c);
        }
    }

    stacks.iter()
        .map(|v|
            v.last().expect("No last string")
        )
        .collect::<String>()
}

fn part_2(stacks: &mut Vec<Vec<char>>, instructions: Vec<(u32, u32, u32)>) -> String {
    for (m ,f ,t) in instructions {
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..m {
            let st_from = stacks.get_mut(f as usize-1).expect("Bad stack");
            let c = st_from.pop().expect("Nothing to get");
            temp.push(c);
        }
        let st_to = stacks.get_mut(t as usize-1).expect("Bad stack");
        temp.reverse();
        st_to.append(&mut temp);
    }

    stacks.iter()
        .map(|v|
            v.last().expect("No last string")
        )
        .collect::<String>()
}

fn parse_input(input: &String) -> (&str, &str) {
    // println!("{input}");
    input.split_once("\n\n").expect("Could not split input")
}

fn main() {
    let input = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");

    let (stacks, instr) = parse_input(&input);
    let ans = part_1(
        &mut make_stacks(parse_lines_to_tuple(stacks)),
        instructions(instr)
    );

    println!("The answer for the first part: {}", ans);

    let ans = part_2(
        &mut make_stacks(parse_lines_to_tuple(stacks)),
        instructions(instr)
    );

    println!("The answer for the second part: {}", ans);

}
