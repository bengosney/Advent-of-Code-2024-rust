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
    assert_eq!(part2, Result::Ok(6));
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

const fn add(a: Point, b: Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
}

fn walk_path(map: &HashMap<Point, char>, start_position: Point) -> Result<HashSet<Point>, &'static str> {
    let mut position = start_position.clone();
    let mut direction = (0, -1);
    let mut visited_directions: HashMap<Point, HashSet<Point>> = HashMap::new();

    loop {
        if visited_directions.entry(position).or_default().contains(&direction) {
            break Err("Loop detected");
        }
        visited_directions.entry(position).or_default().insert(direction); 

        match map.get(&add(position, direction)) {
            Some('#') => direction = turn_left(direction).unwrap(),
            Some(_) => position = add(position, direction),
            None => break Ok(visited_directions.keys().cloned().collect()),
        }
    }
}

pub fn part1(input: &str) -> Result<i32, &'static str> {
    let (map, start_position) = process_input(input);

    match walk_path(&map, start_position) {
        Ok(visited) => Ok(visited.len() as i32),
        Err(e) => Err(e),
    }
}

pub fn part2(input: &str) -> Result<i32, &'static str> {
    let (mut map, start_position) = process_input(input);

    let visited = match walk_path(&map, start_position) {
        Ok(visited) => visited,
        Err(e) => return Err(e),
    };
    let mut loops = 0;

    for &obstruction_position in visited.iter() {
        let original_value = map.insert(obstruction_position, '#');
        if walk_path(&map, start_position).is_err() {
            loops += 1;
        };
        if let Some(value) = original_value {
            map.insert(obstruction_position, value);
        }
    }

    Ok(loops)
}
