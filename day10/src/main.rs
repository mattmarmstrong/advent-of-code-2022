
enum Ops {
    Noop, 
    AddX(i32)
}
fn parse<'input>(input: &'input str) -> Vec<Ops>  {
    input
    .lines()
    .map(|l| {
        let mut l_iter = l.split_ascii_whitespace();
        let op = l_iter.next().unwrap();
        match op {
            "noop" => Ops::Noop,
            "addx" => Ops::AddX(l_iter.next().unwrap().parse::<i32>().unwrap()),
            _ => unreachable!()
        }
    })
    .collect()
}

fn do_ops(ops_vec: &Vec<Ops>) -> Vec<i32> {
    let mut cycle_clock: usize = 1;
    let mut register_val: i32 = 1;
    let mut reg_values_per_cycle = vec![];
    reg_values_per_cycle.push(register_val);
    ops_vec
    .iter()
    .for_each(|op| {
        match *op {
            Ops::Noop => { cycle_clock += 1; reg_values_per_cycle.push(register_val);},
            Ops::AddX(val) => { 
                cycle_clock += 1;
                reg_values_per_cycle.push(register_val);
                cycle_clock += 1;
                register_val += val;
                reg_values_per_cycle.push(register_val);
            }
        }
    });
    reg_values_per_cycle
}

fn part_one(ops_vec: &Vec<Ops> ) -> i32 {
    let reg_vec = do_ops(ops_vec);
    let (_, rest) = reg_vec.split_at(19);
    rest
    .iter()
    .enumerate()
    .filter_map(|(i, val)| {
        if i % 40 == 0 {
            Some((i + 20) as i32 * val)
        } else {
            None
        }
    })
    .sum()
}

fn part_two(ops_vec: &Vec<Ops>) -> String {
    let reg_vec = do_ops(ops_vec);
    let mut screen = String::with_capacity(40 * 6);
    reg_vec
    .iter()
    .enumerate()
    .for_each(|(i, val)| { 
        if (i % 40) as i32 >= val - 1 && (i % 40) as i32 <= val + 1 {
            screen.push('#')
        } else {
            screen.push(' ')
        }
    });
    screen
    .chars()
    .enumerate()
    .flat_map(|(i, c)| {
        if i != 0 && i % 40 == 0 {
            Some('\n')
        } else {
            None
        }
        .into_iter()
        .chain(std::iter::once(c))
    })
    .collect::<String>()

}

fn main() {
    let parsed_input = parse(include_str!("../input.txt"));
    println!("part 1 answer: {}", part_one(&parsed_input));
    println!("part 2 answer: \n {}", part_two(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn test_part_one() {
        let test_input = parse(include_str!("../example.txt"));
        assert_eq!(part_one(&test_input), 13140);
    }

    #[test]
    fn test_part_two() {
        // let test_input = parse(include_str!("../example.txt"));
        //assert_eq!(part_two(&test_input), 0);
    }
}
