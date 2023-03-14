use std::u32;

fn parse(input: &str) -> Vec<(u32, u32)> {
    input
    .lines()
    .map(|x| {
        let bytes = x.as_bytes(); //A = 65 and X = 88
        ((bytes.first().unwrap() - 64) as u32, ((bytes.last().unwrap() - 87) as u32))
    })
    .collect() 
}

fn part_one(input: &Vec<(u32, u32)>) -> u32 {
    // 1 = rock, 2 = paper, 3 = scissors
    input
    .iter()
    .map(|&(theirs, ours)| {
        match 3 + theirs - ours {
            1 | 4 => ours, // lose
            2 | 5 => ours + 6, // win
            3 => ours + 3, // draw  
            _ => unreachable!()
        }})
        .sum::<u32>()
        

}

fn part_two(input: &Vec<(u32, u32)>) -> u32 {
    input
    .iter()
    .map(|&(theirs, desired_result)| {
        match desired_result { 
                1 => match theirs {
                        1 => 3,
                        2 => 1,
                        3 => 2,
                    _ => unreachable!()}
                2  => theirs + 3, 
                3 => 6 + match theirs {
                    1 => 2,
                    2 => 3,
                    3 => 1,
                    _ => unreachable!()
                }, 
                _ => unreachable!()
        }
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
    const SAMPLE: &str = "A Y
B X
C Z";

    #[test]
    fn test_part_one() {
        let test_input = parse(SAMPLE);
        assert_eq!(part_one(&test_input), 15)
    }

    #[test]
    fn test_part_two() {
        let test_input = parse(SAMPLE);
        assert_eq!(part_two(&test_input), 12)
    }
}
