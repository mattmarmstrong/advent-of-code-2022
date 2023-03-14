fn parse(input: &str) -> Vec<char> {
    input
    .chars()
    .collect()
}

fn find_marker_position(input: &Vec<char>, window_size: usize) -> Option<usize> {
    for (pos, chars) in input.windows(window_size).enumerate() {
        if !chars.iter().enumerate().any(|(i, c)| chars[i +1..].contains(c)) {
            return Some(pos + window_size);
        }
    }
    None
}

fn part_one(input: &Vec<char>) -> usize {
    find_marker_position(&input, 4).unwrap()
}

fn part_two(input: &Vec<char>) -> usize {
    find_marker_position(&input, 14).unwrap() 
}

fn main() {
    let parsed_input = parse(include_str!("../input.txt"));
    println!("part 1 answer: {}", part_one(&parsed_input));
    println!("part 2 answer: {}", part_two(&parsed_input));
}

#[cfg(test)]

mod tests {
    use super::*;
   
    #[test]
    fn test_part_one() {
        let test_input = parse(include_str!("../example_part1.txt"));
        assert_eq!(part_one(&test_input), 7);
    }

    #[test]
    fn test_part_two() {
        let test_input = parse(include_str!("../example_part2.txt"));
        assert_eq!(part_two(&test_input), 19);
    }
}
