use crate::utils::PuzzleResult;

#[cfg(test)]
const EXAMPLE_INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

#[test]
fn test_part1_example() {
    let part1 = part1(EXAMPLE_INPUT);
    assert_eq!(part1, Result::Ok(2));
}

#[test]
fn test_part2_example() {
    let part2 = part2(EXAMPLE_INPUT);
    assert_eq!(part2, Result::Ok(4));
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn direction(a: i32, b: i32) -> i32 {
    (a < b) as i32 - (a > b) as i32
}

#[test]
fn test_pairwise() {
    let row = vec![1, 2, 3, 4];
    let pairs = pairwise(&row);
    assert_eq!(pairs, vec![(1, 2), (2, 3), (3, 4)]);
}

fn pairwise(row: &Vec<i32>) -> Vec<(i32, i32)> {
    row.iter()
        .zip(row.iter().skip(1))
        .map(|(&a, &b)| (a, b))
        .collect()
}

fn check_row(row: &Vec<i32>) -> bool {
    let mut row_dir = 0;
    for (a, b) in pairwise(row) {
        let dir = direction(a, b);
        if ![0, dir].contains(&row_dir) || ![1, 2, 3].contains(&(a - b).abs()) {
            return false;
        }
        row_dir = dir;
    }

    return true;
}

fn check_row_with_damper(row: &Vec<i32>) -> bool {
    for i in 0..row.len() {
        let mut row = row.clone();
        row.remove(i);
        if check_row(&row) {
            return true;
        }
    }

    return false;
}

pub fn part1(input: &str) -> PuzzleResult {
    let rows = parse_input(input);
    let count = rows.iter().filter(|row| check_row(row)).count();
    Ok(count as usize)
}

pub fn part2(input: &str) -> PuzzleResult {
    let rows = parse_input(input);
    let count = rows
        .iter()
        .filter(|row| check_row(row) || check_row_with_damper(row))
        .count();
    Ok(count as usize)
}
