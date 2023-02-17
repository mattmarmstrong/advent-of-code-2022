fn parse(input: &str) -> () {
}

fn part_one() -> () {
}

fn part_two() -> () {

}

fn main() {
    let parsed_input = parse(include_str!("../input.txt"));
    //println!("part 1 answer: {}", part_one(&parsed_input));
    //println!("part 2 answer: {}", part_two(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn test_part_one() {
        let test_input = parse(include_str!("../example.txt"));
        //assert_eq!(part_one(&test_input), 0);
    }

    #[test]
    fn test_part_two() {
        let test_input = parse(include_str!("../example.txt"));
        //assert_eq!(part_two(&test_input), 0);
    }
}
