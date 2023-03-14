use itertools::Itertools;
use std::cmp::Reverse;
use std::u32;

fn parse(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|x| x.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
        .collect()
}

fn part_one(elves: &Vec<u32>) -> u32 {
    elves.iter().max().unwrap().clone() //u32 implements clone
}

fn part_two(elves: &Vec<u32>) -> u32 {
    elves
        .iter()
        .sorted_by_key(|x| Reverse(*x))
        .take(3)
        .sum::<u32>()
}

fn main() {
    println!("{:?}", "\n\n".parse::<u32>().ok());
    let parsed_input = parse(include_str!("../input.txt"));
    println!("part 1 answer: {}", part_one(&parsed_input));
    println!("part 2 answer: {}", part_two(&parsed_input));
}

#[cfg(test)]

mod tests {
    use super::*;
    const SAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part_one() {
        let test_input = parse(SAMPLE);
        assert_eq!(part_one(&test_input), 24000)
    }

    #[test]
    fn test_part_two() {
        let test_input = parse(SAMPLE);
        assert_eq!(part_two(&test_input), 45000)
    }
}
