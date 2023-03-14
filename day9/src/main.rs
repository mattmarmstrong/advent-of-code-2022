use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn parse(input: &str) -> impl Iterator<Item = (Direction, u32)> + '_ {
    input.lines().map(|l| {
        let mut line_iter = l.split_ascii_whitespace();
        let dir = line_iter.next().unwrap();
        let distance = line_iter.next().unwrap().parse::<u32>().unwrap();
        match dir {
            "U" => (Direction::Up, distance),
            "D" => (Direction::Down, distance),
            "L" => (Direction::Right, distance),
            "R" => (Direction::Left, distance),
            _ => unreachable!(),
        }
    })
}

fn move_rope(instruction_iter: &mut Box<impl Iterator<Item = (Direction, u32)>>, rope_len: usize) -> usize {
    let mut knots_position_vec = vec![(0, 0); rope_len];
    let mut visited_postions: HashSet<(i32, i32)> = HashSet::new();
    instruction_iter.for_each(|(dir, move_size)| {
        for _ in 0..move_size {
            match dir {
                Direction::Up => knots_position_vec[0].1 += 1,
                Direction::Down => knots_position_vec[0].1 -= 1,
                Direction::Right => knots_position_vec[0].0 += 1,
                Direction::Left => knots_position_vec[0].0 -= 1,
            }
            for i in 0..rope_len - 1 {
                let (x_1, y_1) = knots_position_vec[i];
                let (x_2, y_2) = &mut knots_position_vec[i + 1];
                let (delta_x, delta_y): (i32, i32) = (x_1 - *x_2, y_1 - *y_2);
                let is_adjacent = !(delta_x.abs() > 1 || delta_y.abs() > 1) as i32;
                if x_2 < &mut (x_1 - is_adjacent) { // ropes going right
                    *x_2 += 1
                }
                if x_2 > &mut (x_1 + is_adjacent) { // ropes going left
                    *x_2 -= 1
                }
                if y_2 < &mut (y_1 - is_adjacent) { // ropes going up
                    *y_2 += 1;
                }
                if y_2 > &mut (y_1 + is_adjacent) { // ropes going down 
                    *y_2 -= 1;
                }}
            visited_postions.insert((knots_position_vec.last().unwrap().0, knots_position_vec.last().unwrap().1));
        }
    });
    visited_postions.len()
}

fn part_one(instruct_iter: &mut Box<impl Iterator<Item = (Direction, u32)>>) -> usize {
    move_rope(instruct_iter, 2)
}

fn part_two(instruct_iter: &mut Box<impl Iterator<Item = (Direction, u32)>>) -> usize {
    move_rope(instruct_iter, 10)
}

fn main() {
    let mut parsed_input_p1 = Box::new(parse(include_str!("../input.txt")));
    let mut parsed_input_p2 = Box::new(parse(include_str!("../input.txt")));
    println!("part 1 answer: {}", part_one(&mut parsed_input_p1));
    println!("part 2 answer: {}", part_two(&mut parsed_input_p2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut test_input = Box::new(parse(include_str!("../example.txt")));
        assert_eq!(part_one(&mut test_input), 13);
    }

    #[test]
    fn test_part_two() {
        let mut test_input = Box::new(parse(include_str!("../example.txt")));
        assert_eq!(part_two(&mut test_input), 1);
    }
}
