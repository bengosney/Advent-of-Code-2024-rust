use std::collections::{HashMap, HashSet};

#[cfg(test)]
const EXAMPLE_INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

#[test]
fn test_part1_example() {
    let part1 = part1(EXAMPLE_INPUT);
    assert_eq!(part1, Result::Ok(41));
}

#[test]
fn test_part2_example() {
    let part2 = part2(EXAMPLE_INPUT);
    assert_eq!(part2, Result::Ok(0));
}

type Point = (i32, i32);

fn process_input(input: &str) -> (HashMap<Point, char>, Point) {
    let mut map = HashMap::new();
    let mut start_position = (0, 0);

    for (y, row) in input.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            map.insert(
                (x as i32, y as i32),
                match cell {
                    '.' => '.',
                    '#' => '#',
                    '^' => {
                        start_position = (x as i32, y as i32);
                        '.'
                    }
                    _ => panic!("Invalid cell: {}", cell),
                },
            );
        }
    }

    (map, start_position)
}

const fn turn_left(direction: Point) -> Result<Point, &'static str> {
    match direction {
        (0, -1) => Ok((1, 0)),
        (1, 0) => Ok((0, 1)),
        (0, 1) => Ok((-1, 0)),
        (-1, 0) => Ok((0, -1)),
        _ => Err("Invalid direction"),
    }
}

fn walk_path(map: HashMap<Point, char>, start_position: Point) -> HashSet<Point> {
    let mut position = start_position.clone();
    let mut direction = (0, -1);
    let mut visited = HashSet::new();

    loop {
        visited.insert(position.clone());

        match map.get(&(position.0 + direction.0, position.1 + direction.1)) {
            Some('#') => direction = turn_left(direction).unwrap(),
            Some(_) => position = (position.0 + direction.0, position.1 + direction.1),
            None => break visited,
        }
    }
}

pub fn part1(input: &str) -> Result<i32, &'static str> {
    let (map, start_position) = process_input(input);

    let visited = walk_path(map, start_position);

    Ok(visited.len() as i32)
}

pub fn part2(_input: &str) -> Result<i32, &'static str> {
    Ok(0)
}
