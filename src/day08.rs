use crate::utils::PuzzleResult;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Add;
use std::ops::Sub;

#[cfg(test)]
const EXAMPLE_INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

#[test]
fn test_part1_example() {
    let part1 = part1(EXAMPLE_INPUT);
    assert_eq!(part1, Result::Ok(14));
}

#[test]
fn test_part2_example() {
    let part2 = part2(EXAMPLE_INPUT);
    assert_eq!(part2, Result::Ok(34));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn in_bounds(&self, extents: Point) -> bool {
        (self.x >= 0 && self.x < extents.x) && (self.y >= 0 && self.y < extents.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn parse_input(input: &str) -> (HashMap<char, HashSet<Point>>, Point) {
    let mut map: HashMap<char, HashSet<Point>> = HashMap::new();
    let mut extents = Point::new(0, 0);

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                map.entry(c)
                    .or_default()
                    .insert(Point::new(x as isize, y as isize));
            }
            extents.x = extents.x.max(x as isize + 1);
        });
        extents.y = extents.y.max(y as isize + 1);
    });

    return (map, extents);
}

pub fn part1(input: &str) -> PuzzleResult {
    let (map, extents) = parse_input(input);

    Ok(map
        .values()
        .flat_map(|points| {
            points.iter().combinations(2).into_iter().flat_map(|pair| {
                let (a, b) = (pair[0], pair[1]);
                let delta = *a - *b;
                vec![*a + delta, *b - delta]
            })
        })
        .filter(|p| p.in_bounds(extents))
        .collect::<HashSet<Point>>()
        .len())
}

fn calculate_nodes(start: Point, direction: Point, extents: Point) -> HashSet<Point> {
    std::iter::successors(Some(start), |&p| Some(p + direction))
        .take_while(|p| p.in_bounds(extents))
        .collect()
}

pub fn part2(input: &str) -> PuzzleResult {
    let (map, extents) = parse_input(input);

    Ok(map
        .values()
        .flat_map(|points| {
            points.iter().combinations(2).into_iter().flat_map(|pair| {
                let (a, b) = (pair[0], pair[1]);
                vec![
                    calculate_nodes(*a, *a - *b, extents),
                    calculate_nodes(*b, *b - *a, extents),
                ]
                .into_iter()
                .flatten()
            })
        })
        .filter(|p| p.in_bounds(extents))
        .collect::<HashSet<Point>>()
        .len())
}
