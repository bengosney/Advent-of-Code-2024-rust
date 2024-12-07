use crate::utils::PuzzleResult;

#[cfg(test)]
const EXAMPLE_INPUT: &str = r#""#;

#[test]
fn test_part1_example() {
    let part1 = part1(EXAMPLE_INPUT);
    assert_eq!(part1, Result::Ok(0));
}

#[test]
fn test_part2_example() {
    let part2 = part2(EXAMPLE_INPUT);
    assert_eq!(part2, Result::Ok(0));
}

pub fn part1(_input: &str) -> PuzzleResult {
    Err("Not implemented".into())
}

pub fn part2(_input: &str) -> PuzzleResult {
    Err("Not implemented".into())
}
