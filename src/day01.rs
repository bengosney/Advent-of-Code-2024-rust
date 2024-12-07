use std::iter::zip;
use crate::utils::PuzzleResult;

#[cfg(test)]
const EXAMPLE_INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

#[test]
fn test_part1_example() {
    let part1 = part1(EXAMPLE_INPUT);
    assert_eq!(part1, Result::Ok(11));
}

#[test]
fn test_part2_example() {
    let part2 = part2(EXAMPLE_INPUT);
    assert_eq!(part2, Result::Ok(31));
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            (numbers[0], numbers[1])
        })
        .unzip()
}

pub fn part1(input: &str) -> PuzzleResult {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    Ok(zip(left, right).map(|(l, r)| (r - l).abs()).sum::<i32>() as usize)
}

fn count_items<T: PartialEq>(needle: &T, haystack: &Vec<T>) -> i32 {
    haystack.iter().filter(|&x| *x == *needle).count() as i32
}

pub fn part2(input: &str) -> PuzzleResult {
    let (left, right) = parse_input(input);

    Ok(left.iter().map(|l| l * count_items(l, &right) as i32).sum::<i32>() as usize)
}
