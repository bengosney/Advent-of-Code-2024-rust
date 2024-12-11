use crate::utils::{PuzzleResult, point::Point};
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

#[cfg(test)]
const EXAMPLE_INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

#[test]
fn test_part1_example() {
    let part1 = part1(EXAMPLE_INPUT);
    assert_eq!(part1, Result::Ok(36));
}

#[test]
fn test_part2_example() {
    let part2 = part2(EXAMPLE_INPUT);
    assert_eq!(part2, Result::Ok(81));
}

fn parse_input(input: &str) -> HashMap<Point, i32> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    Point::new(x as isize, y as isize),
                    c.to_digit(10).unwrap() as i32,
                )
            })
        })
        .collect()
}

fn walk_trail(map: &HashMap<Point, i32>, start: Point) -> i32 {
    let mut found: HashSet<Point> = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);

    while let Some(point) = queue.pop_front() {
        let current = map.get(&point).copied().unwrap_or(0);
        if current == 9 {
            found.insert(point);
            continue;
        }

        point
            .neighbors()
            .into_iter()
            .filter(|neighbor| map.get(neighbor).copied().unwrap_or(0) - current == 1)
            .for_each(|neighbor| queue.push_back(neighbor));
    }

    found.len() as i32
}

pub fn part1(input: &str) -> PuzzleResult {
    let map = parse_input(input);

    Ok(map
        .iter()
        .filter(|(_, &v)| v == 0)
        .map(|(k, _)| walk_trail(&map, *k))
        .sum::<i32>() as usize)
}

fn count_trails(map: &HashMap<Point, i32>, position: Point) -> i32 {
    let current = map.get(&position).copied().unwrap_or(0);
    if current == 9 {
        return 1;
    }

    position
        .neighbors()
        .into_iter()
        .filter(|neighbor| map.get(neighbor).copied().unwrap_or(0) - current == 1)
        .map(|neighbor| count_trails(map, neighbor))
        .sum::<i32>()
}

pub fn part2(input: &str) -> PuzzleResult {
    let map = parse_input(input);

    Ok(map
        .iter()
        .filter(|(_, &v)| v == 0)
        .map(|(k, _)| count_trails(&map, *k))
        .sum::<i32>() as usize)
}
