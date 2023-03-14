use std::{cell::RefCell, collections::VecDeque};

struct Monkey {
    items: VecDeque<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    divider_test: usize,
    if_true: usize,
    if_false: usize,
    inspection_count: usize,
}

fn parse(input: &str) -> Vec<RefCell<Monkey>> {
    input
    .split("\n\n")
    .map(|monkey| {
        let mut l = monkey.lines().skip(1).map(|l| l.trim());
        Monkey {
            items: l
                .next()
                .unwrap()
                .trim_start_matches("Starting items: ")
                .split(',')
                .map(|i| i.trim().parse().unwrap())
                .collect(),
            op: {
                let mut e = l
                    .next()
                    .unwrap()
                    .trim_start_matches("Operation: new = old ")
                    .split(' ');
                let op = e.next().unwrap();
                let val = e.next().unwrap().parse::<usize>();
                if let Ok(val) = val {
                    match op {
                        "+" => Box::new(move |i| i + val),
                        "*" => Box::new(move |i| i * val),
                        _ => unreachable!(),
                    }
                } else {
                    match op {
                        "+" => Box::new(move |i| i + i),
                        "*" => Box::new(move |i| i * i),
                        _ => unreachable!(),
                    }
                }
            },
            divider_test: l
                .next()
                .unwrap()
                .trim_start_matches("Test: divisible by ")
                .parse()
                .unwrap(),
            if_true: l
                .next()
                .unwrap()
                .trim_start_matches("If true: throw to monkey ")
                .parse()
                .unwrap(),
            if_false: l
                .next()
                .unwrap()
                .trim_start_matches("If false: throw to monkey ")
                .parse()
                .unwrap(),
            inspection_count: 0,
        }
    })
    .map(RefCell::new)
    .collect()
    }


fn do_n_rounds(monkey_vec: &mut [RefCell<Monkey>], n: usize, reduced_worry: bool) -> () {
    let num_monkeys = monkey_vec
    .iter()
    .map(|m| m.borrow().divider_test)
    .product::<usize>();

    for _ in 0..n { 
        let _ = &monkey_vec
        .iter()
        .for_each(|m| {
            let mut m_ref = m.borrow_mut();
            let monkey = &mut *m_ref;
            while let Some(mut item) = monkey.items.pop_front() {
                monkey.inspection_count += 1;
                item = (monkey.op)(item);
                if reduced_worry {
                item /= 3;
                }
                item %= num_monkeys;
                let target = match item % monkey.divider_test == 0 {
                    true => monkey.if_true,
                    false => monkey.if_false 
                };
                monkey_vec[target].borrow_mut().items.push_back(item)
            }
        });
    }
    monkey_vec.sort_by_key(|m| m.borrow().inspection_count);


    
}

fn part_one(monkey_vec: &mut [RefCell<Monkey>]) -> usize {
    do_n_rounds(monkey_vec, 20, true);
    monkey_vec
    .iter()
    .map(|m| m.borrow().inspection_count)
    .rev()
    .take(2)
    .product::<usize>()
}

fn part_two(monkey_vec: &mut [RefCell<Monkey>]) -> usize {
    do_n_rounds(monkey_vec, 10000, false);
    monkey_vec
    .iter()
    .map(|m| m.borrow().inspection_count)
    .rev()
    .take(2)
    .product::<usize>()
}

fn main() {
    let mut parsed_input_p1 = parse(include_str!("../input.txt"));
    let mut parsed_input_p2 = parse(include_str!("../input.txt"));
    println!("part 1 answer: {}", part_one(&mut parsed_input_p1));
    println!("part 2 answer: {}", part_two(&mut parsed_input_p2));
}

#[cfg(test)]
mod tests {
    // use super::*; 
   
    #[test]
    fn test_part_one() {
        // let test_input = parse(include_str!("../example.txt"));
        //assert_eq!(part_one(&test_input), 0);
    }

    #[test]
    fn test_part_two() {
        // let test_input = parse(include_str!("../example.txt"));
        //assert_eq!(part_two(&test_input), 0);
    }
}
