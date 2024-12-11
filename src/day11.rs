use crate::utils::PuzzleResult;
use cached::proc_macro::cached;

#[cfg(test)]
const EXAMPLE_INPUT: &str = r#"125 17"#;

#[test]
fn test_part1_example() {
    let part1 = part1(EXAMPLE_INPUT);
    assert_eq!(part1, Result::Ok(55312));
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cached]
fn blink(stone: i64, iterations: i32) -> i64 {
    if iterations == 0 {
        return 1;
    }

    let value = if stone == 0 {
        blink(1, iterations - 1)
    } else {
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            let mid = stone_str.len() / 2;
            let (first_half, second_half) = stone_str.split_at(mid);
            blink(first_half.parse().unwrap(), iterations - 1)
                + blink(second_half.parse().unwrap(), iterations - 1)
        } else {
            blink(stone * 2024, iterations - 1)
        }
    };

    value
}

pub fn part1(input: &str) -> PuzzleResult {
    Ok(parse_input(input)
        .into_iter()
        .map(|stone| blink(stone, 25))
        .sum::<i64>() as usize)
}

pub fn part2(input: &str) -> PuzzleResult {
    Ok(parse_input(input)
        .into_iter()
        .map(|stone| blink(stone, 75))
        .sum::<i64>() as usize)
}
