use ::rayon::prelude::*;
use take_until::TakeUntilExt;

type TreeGrid = Vec<Vec<u32>>;

fn parse(input: &str) -> TreeGrid {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn tree_iter(grid: &TreeGrid) -> impl ParallelIterator<Item = (u32, (usize, usize))> + '_ {
    grid.par_iter()
        .enumerate()
        .flat_map(|(y, l)| l.par_iter().enumerate().map(move |(x, v)| (*v, (x, y))))
}

fn line_col_iter<'a>(
    grid: &'_ TreeGrid,
    (x, y): (usize, usize),
) -> impl Iterator<Item = impl Iterator<Item = u32> + '_> {
    let up = Box::new((0..y).rev().map(move |i| grid[i][x]));
    let down = Box::new((y + 1..grid.len()).map(move |i| grid[i][x]));
    let left = Box::new((0..x).rev().map(move |i| grid[y][i]));
    let right = Box::new((x + 1..grid[0].len()).map(move |i| grid[y][i]));

    let a: [Box<dyn Iterator<Item = u32> + '_>; 4] = [up, down, left, right];
    a.into_iter()
}

fn part_one(tree_grid: &TreeGrid) -> usize {
    tree_iter(&tree_grid)
        .filter(|(current, (x, y))| {
            line_col_iter(&tree_grid, (*x, *y)).any(|mut line| line.all(|h| h < *current))
        })
        .count()
}

fn part_two(tree_grid: &TreeGrid) -> usize {
    tree_iter(&tree_grid)
        .map(|(current, (x, y))| {
            line_col_iter(&tree_grid, (x, y))
                .map(|line| line.take_until(|h| *h >= current).count())
                .product::<usize>()
        })
        .max()
        .unwrap()
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
        let test_input = parse(include_str!("../example.txt"));
        assert_eq!(part_one(&test_input), 21);
    }

    #[test]
    fn test_part_two() {
        let test_input = parse(include_str!("../example.txt"));
        assert_eq!(part_two(&test_input), 8);
    }
}
