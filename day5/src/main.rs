use itertools::Itertools;

type Stacks = Vec<Vec<char>>;
type Moves = Vec<(usize, usize, usize)>;

fn parse(input: &str) -> (Stacks, Moves) {
    let (stacks_str, moves_str) = input.split_once("\n\n").unwrap();
    let mut stacks_iter = stacks_str.lines().rev();
    let num_stacks = stacks_iter.next().unwrap().replace(" ", "").len();
    let mut stacks: Stacks = vec![vec![]; num_stacks];

    stacks_iter.for_each(|stack_level| {
        stack_level.chars().skip(1).enumerate().for_each(|(i, c)| {
            if i % 4 == 0 && c != ' ' {
                stacks[i / 4].push(c);
            }
        });
    });

    let moves: Vec<(usize, usize, usize)> = moves_str
        .lines()
        .filter_map(|m| {
            let s: Vec<&str> = m.split_ascii_whitespace().collect();
            Some((s[1].parse().ok()?, s[3].parse().ok()?, s[5].parse().ok()?))
        })
        .collect();

    (stacks, moves)
}

fn do_moves(s: &mut Stacks, m: &Moves, version: u32) -> String {
    m.iter().for_each(|(n, from, to)| {
        let stack = &mut s[*from - 1];
        let crates_to_move = stack.split_off(stack.len() - n);
        if version == 9000 {
            s[*to - 1].extend(crates_to_move.iter().rev());
        } else {
            s[*to - 1].extend(crates_to_move.iter());
        }
    });
    s.iter_mut().filter_map(|stack| stack.last()).join("")
}

fn part_one(s: &mut Stacks, m: &Moves) -> String {
    do_moves(s, m, 9000)
}

fn part_two(s: &mut Stacks, m: &Moves) -> String {
    do_moves(s, m, 9001)
}

fn main() {
    let mut parsed_input = parse(include_str!("../input.txt"));
    let mut stack_2 = parsed_input.0.clone();
    println!(
        "part 1 answer: {}",
        part_one(&mut parsed_input.0, &parsed_input.1)
    );
    println!("part 2 answer: {}", part_two(&mut stack_2, &parsed_input.1));
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut test_input = parse(include_str!("../example.txt"));
        assert_eq!(part_one(&mut test_input.0, &test_input.1), "CMZ");
    }

    #[test]
    fn test_part_two() {
        let mut test_input = parse(include_str!("../example.txt"));
        assert_eq!(part_two(&mut test_input.0, &test_input.1), "MCD");
    }
}
