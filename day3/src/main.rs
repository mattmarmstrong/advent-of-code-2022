use std::u32;

fn parse(input: &str) -> Vec<&str> {
    input
    .lines()
    .collect()

}

fn byte_priority(byte: &u8) -> u32 {
    match byte {
        97..=122 => (byte - 96) as u32, // normalize based on uppercase/lowercase UTF-8 encoding,
        65..=90 => (26 + byte - 64) as u32,
        _ => panic!()
    }
}

fn part_one(input: &Vec<&str>) -> u32 {
    input
    .iter()
    .filter_map(|&x| 
        {
        let comps = x.split_at(x.len() / 2);
        let comp1 = comps.0.as_bytes();
        let comp2 = comps.1.as_bytes();
        comp1
        .iter()
        .find(|&byte| comp2.contains(byte))
        .map(|byte| byte_priority(byte))
    })
    .sum::<u32>()

}

fn part_two(input: &Vec<&str>) -> u32 {
    input
    .chunks(3)
    .filter_map(|x| {
        let mut elves_iter = x.iter();
        let elf_1 = elves_iter.next().unwrap().as_bytes();
        let elf_2 = elves_iter.next().unwrap().as_bytes();
        let elf_3 = elves_iter.next().unwrap().as_bytes(); 
        elf_1
        .iter()
        .find(|&byte| elf_2.contains(byte) && elf_3.contains(byte))
        .map(|byte| byte_priority(byte))

    })
    .sum::<u32>()

}

fn main() {
    let parsed_input = parse(include_str!("../input.txt"));
    println!("part 1 answer: {}", part_one(&parsed_input));
    println!("part 2 answer: {}", part_two(&parsed_input));
}

#[cfg(test)]

mod tests {
    use super::*;
    const SAMPLE: &str = 
    "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_one() {
        let test_input = parse(SAMPLE);
        assert_eq!(part_one(&test_input), 157)
    }

    #[test]
    fn test_part_two() {
        let test_input = parse(SAMPLE);
        assert_eq!(part_two(&test_input), 70)
    }
}
