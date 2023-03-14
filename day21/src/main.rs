use std::collections::HashMap;

#[derive(Debug)]
enum Phrase {
    Number(isize),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}


fn compute_phrase(monkeys: &HashMap<String, Phrase>, monkey_name: &str) -> isize { 
    match &monkeys[monkey_name] {
        Phrase::Number(num) => *num,
        Phrase::Add(monkey_1,  monkey_2) => compute_phrase(monkeys, monkey_1) + compute_phrase(monkeys, monkey_2),
        Phrase::Subtract(monkey_1,  monkey_2) => compute_phrase(monkeys, monkey_1) - compute_phrase(monkeys, monkey_2),
        Phrase::Multiply(monkey_1,  monkey_2) => compute_phrase(monkeys, monkey_1) * compute_phrase(monkeys, monkey_2),
        Phrase::Divide(monkey_1,  monkey_2) => compute_phrase(monkeys, monkey_1) / compute_phrase(monkeys, monkey_2),
        }
    }


fn parse(input: &str) -> HashMap<String, Phrase> {
    let mut monkeys: HashMap<String, Phrase> = HashMap::new();
    input
    .lines()
    .for_each(|l| {
        let mut line_iter = l.split(": ");
        let (monkey_name, phrase_str) = (line_iter.next().unwrap(), line_iter.next().unwrap());
        let mut phrase_vec: Vec<_> = phrase_str.split_ascii_whitespace().collect();
        let mut phrase = Phrase::Number(0); // I dont like this 
        if phrase_vec.len() == 1 {
            let phrase_num = phrase_vec.pop().unwrap().parse::<isize>().unwrap();
            phrase = Phrase::Number(phrase_num); 
        } else {
            // pop takes from the back of a Vec 
            let (monkey_2, op_str, monkey_1) = (phrase_vec.pop().unwrap(), phrase_vec.pop().unwrap(), phrase_vec.pop().unwrap());
            match op_str {
                "+" => { phrase = Phrase::Add(monkey_1.to_string(), monkey_2.to_string());}
                "-" => { phrase = Phrase::Subtract(monkey_1.to_string(), monkey_2.to_string());}
                "*" => { phrase = Phrase::Multiply(monkey_1.to_string(), monkey_2.to_string());}
                "/" => { phrase = Phrase::Divide(monkey_1.to_string(), monkey_2.to_string());},
                _ => unreachable!()
            }
        }
        monkeys.insert(monkey_name.to_string(), phrase);
        });
        monkeys
    }

fn part_one(monkeys: &HashMap<String, Phrase>, monkey_name: &str) -> isize {
    compute_phrase(monkeys, monkey_name)
}

fn part_two() -> () {

}

fn main() {
    let parsed_input = parse(include_str!("../input.txt"));
    println!("part 1 answer: {}", part_one(&parsed_input, "root"));
    //println!("part 2 answer: {}", part_two(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn test_part_one() {
        let test_input = parse(include_str!("../example.txt"));
        println!("{:#?}", &test_input);
        assert_eq!(part_one(&test_input, "root"), 152);
    }

    #[test]
    fn test_part_two() {
        let test_input = parse(include_str!("../example.txt"));
        //assert_eq!(part_two(&test_input), 0);
    }
}
