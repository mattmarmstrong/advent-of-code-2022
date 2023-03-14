use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

impl From<&str> for Packet {
    fn from(packet_str: &str) -> Self {
        match &packet_str[0..1] {
            "[" => {
                let mut stack_counter = 0;
                Packet::List(
                    packet_str[1..packet_str.len() - 1]
                        .split(|c| {
                            if c == '[' {
                                stack_counter += 1
                            } else if c == ']' {
                                stack_counter -= 1
                            }
                            c == ',' && stack_counter == 0
                        })
                        .filter_map(|s| (!s.is_empty()).then(|| Self::from(s)))
                        .collect(),
                )
            }
            _ => Packet::Integer(packet_str.parse().unwrap()),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(left), Packet::Integer(right)) => left.cmp(right),
            (Packet::Integer(_), _) => Packet::List(vec![self.clone()]).cmp(other),
            (_, Packet::Integer(_)) => self.cmp(&Packet::List(vec![other.clone()])),
            (Packet::List(left), Packet::List(right)) => left.cmp(right),
        }
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Vec<Packet> {
    input
        .trim()
        .lines()
        .filter_map(|s| (!s.is_empty()).then(|| Packet::from(s)))
        .collect()
}

fn part_one(packets: &Vec<Packet>) -> usize {
    packets
        .chunks(2)
        .into_iter()
        .enumerate()
        .filter(|(_, packet_pair)| packet_pair[0].cmp(&packet_pair[1]) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part_two(packets: &mut Vec<Packet>) -> usize {
    let divider_packets = (Packet::from("[[2]]"), Packet::from("[[6]]"));
    packets.push(divider_packets.0.clone());
    packets.push(divider_packets.1.clone());
    packets.sort();
    let index_1 = packets.iter().position(|p| *p == divider_packets.0).unwrap() + 1;
    let index_2 = packets.iter().position(|p| *p == divider_packets.1).unwrap() + 1;
    index_1 * index_2
}

fn main() {
    let mut parsed_input = parse(include_str!("../input.txt"));
    println!("part 1 answer: {}", part_one(&parsed_input));
    println!("part 2 answer: {}", part_two(&mut parsed_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = parse(include_str!("../example.txt"));
        assert_eq!(part_one(&test_input), 13);
    }

    #[test]
    fn test_part_two() {
        let mut test_input = parse(include_str!("../example.txt"));
        assert_eq!(part_two(&mut test_input), 140);
    }
}
