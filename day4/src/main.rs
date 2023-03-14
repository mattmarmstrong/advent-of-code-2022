use std::ops::RangeInclusive;

fn parse(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    input
        .lines()
        .filter_map(|l| {
            let (r1, r2) = l.split_once(',')?;
            let (r1_start, r1_end) = r1.split_once('-')?;
            let (r2_start, r2_end) = r2.split_once('-')?;
            Some((
                (r1_start.parse::<u32>().ok()?)..=(r1_end.parse::<u32>().ok()?),
                (r2_start.parse::<u32>().ok()?)..=(r2_end.parse::<u32>().ok()?),
            ))
        })
        .collect()
}

fn range_contains_all(range_1: &RangeInclusive<u32>, range_2: &RangeInclusive<u32>) -> u32 {
    (range_1.contains(&range_2.start()) && range_1.contains(&range_2.end())
        || range_2.contains(&range_1.start()) && range_2.contains(&range_1.end())) as u32
}

fn range_contains_some(range_1: &RangeInclusive<u32>, range_2: &RangeInclusive<u32>) -> u32 {
    (range_1.start() <= range_2.start() && range_1.end() >= range_2.start()
        || range_2.start() <= range_1.start() && range_2.end() >= range_1.start()) as u32
}

fn part_one(input: &Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>) -> u32 {
    input
        .iter()
        .filter_map(|(r1, r2)| Some(range_contains_all(r1, r2)))
        .sum::<u32>()
}

fn part_two(input: &Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>) -> u32 {
    input
        .iter()
        .filter_map(|(r1, r2)| Some(range_contains_some(r1, r2)))
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
    const SAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_one() {
        let test_input = parse(SAMPLE);
        assert_eq!(part_one(&test_input), 2);
    }

    #[test]
    fn test_part_two() {
        let test_input = parse(SAMPLE);
        assert_eq!(part_two(&test_input), 4);
    }
}
