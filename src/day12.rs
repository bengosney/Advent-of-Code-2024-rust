use crate::utils::PuzzleResult;
use std::collections::{HashMap,HashSet, VecDeque};


#[cfg(test)]
const EXAMPLE_INPUT: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

#[test]
fn test_part1_example() {
    let part1 = part1(EXAMPLE_INPUT);
    assert_eq!(part1, Result::Ok(1930));
}

#[test]
fn test_part2_example() {
    let part2 = part2(EXAMPLE_INPUT);
    assert_eq!(part2, Result::Ok(0));
}

type Point = (isize, isize);
type Garden = HashMap<Point, char>;

fn parse_input(input: &str) -> Garden {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, cell)| ((x as isize, y as isize), cell))
        })
        .collect()
}

fn get_neighbors(point: &Point) -> Vec<Point> {
    let (x, y) = point;
    vec![
        (x + 1, y + 0),
        (x - 1, y + 0),
        (x + 0, y + 1),
        (x + 0, y - 1),
    ]
}

fn get_perimeter(patch: &HashSet<Point>) -> usize {
    patch.iter()
        .flat_map(|point| get_neighbors(point))
        .filter(|neighbor| !patch.contains(neighbor))
        .count()
}

fn cost_fence(garden: &Garden) -> usize {
    let mut cost: usize = 0;
    let mut visited: HashSet<Point> = HashSet::new();

    for (pos, plant) in garden {
        if visited.contains(pos) {
            continue;
        }

        let mut patch: HashSet<Point> = HashSet::new();
        patch.insert(pos.clone());
        let mut to_check: VecDeque<Point> = VecDeque::new();
        to_check.push_back(*pos);

        while let Some(current) = to_check.pop_front() {
            visited.insert(current.clone());

            for neighbor in get_neighbors(&current) {
                if let Some(&neighbor_plant) = garden.get(&neighbor) {
                    if neighbor_plant == *plant && !visited.contains(&neighbor) {
                        to_check.push_back(neighbor);
                        patch.insert(neighbor);
                        visited.insert(neighbor);
                    }
                }
            }
        }

        cost += patch.len() as usize * get_perimeter(&patch);
    }

    cost
}

pub fn part1(input: &str) -> PuzzleResult {
    let garden = parse_input(input);

    Ok(cost_fence(&garden))
}

pub fn part2(_input: &str) -> PuzzleResult {
    Err("Not implemented".into())
}
