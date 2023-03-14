use std::collections::HashMap;

fn parse(input: &str) -> Vec<&str> {
    input.
    lines()
    .collect()
}

fn build_fs_map(input: &Vec<&str>) -> HashMap<String, usize> {
    let mut path: Vec<&str> = Vec::new();
    let mut fs_map: HashMap<String, usize> = HashMap::new();
    input
    .iter()
    .for_each(|l| {
        let mut command_line = l.split_ascii_whitespace();
        let first = command_line.next().unwrap();
        match first {
            "$" => {
                match command_line.next().unwrap() { 
                    "cd" => {
                        let dir_name = command_line.next().unwrap();
                        match dir_name {
                            "/" => {
                                path.push("$HOME")
                            }
                            ".." => {
                                path.pop();
                            }
                            _ => {
                                path.push(dir_name)
                            }
                        }
                    }
                    _ => {}
                }
            }
            "dir" => {}
            _ => {
                let f_size = first.parse::<usize>().unwrap();
                let mut path_key = "/".to_string();
                path.iter().for_each(|p| {
                    path_key.push_str(p);
                    path_key.push_str("/");
                    let path_key_clone = path_key.clone();
                    let total_size = f_size + fs_map.get(path_key_clone.as_str()).unwrap_or(&0);
                    fs_map.insert(path_key_clone, total_size);
                })
            }
        }
    });
    fs_map
}


fn part_one(fs_map: &HashMap<String, usize>) -> usize {
    let mut sum: usize = 0;
    fs_map
    .keys()
    .for_each(|key| {
        let dir_size = fs_map.get(key.as_str()).unwrap();
        if dir_size <= &100_000 {
            sum += dir_size;
        }
    });
    sum
    

}

fn part_two(fs_map: &HashMap<String, usize>) -> &usize {
    let space_required: usize = 30_000_000 + fs_map.get("/$HOME/").unwrap() - 70_000_000;
    fs_map.keys().filter_map(|key|{
        if fs_map.get(key.as_str()).unwrap() >= &space_required {
            Some(fs_map.get(key.as_str()).unwrap())
        }
        else {
            None
        }
    })
    .min().unwrap()
}


fn main() {
    let parsed_input = parse(include_str!("../input.txt"));
    let fs_map = build_fs_map(&parsed_input);
    println!("part 1 answer: {}", part_one(&fs_map));
    println!("part 2 answer: {}", part_two(&fs_map));
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn test_part_one() {
        let test_input = parse(include_str!("../example.txt"));
        let test_fs_map = build_fs_map(&test_input);
        assert_eq!(part_one(&test_fs_map), 95437);
    }

    #[test]
    fn test_part_two() {
        let test_input = parse(include_str!("../example.txt"));
        let test_fs_map = build_fs_map(&test_input);
        assert_eq!(part_two(&test_fs_map), &24933642);
    }
}
