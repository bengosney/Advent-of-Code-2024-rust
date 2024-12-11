use crate::utils::PuzzleResult;
use std::collections::HashMap;

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

fn blink(stone: i64, iterations: i32, cache: &mut HashMap<(i64, i32), i64>) -> i64 {
    if iterations == 0 {
        return 1;
    }

    if let Some(&result) = cache.get(&(stone, iterations)) {
        return result;
    }

    let value = if stone == 0 {
        blink(1, iterations - 1, cache)
    } else {
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            let mid = stone_str.len() / 2;
            let (first_half, second_half) = stone_str.split_at(mid);
            blink(first_half.parse().unwrap(), iterations - 1, cache)
                + blink(second_half.parse().unwrap(), iterations - 1, cache)
        } else {
            blink(stone * 2024, iterations - 1, cache)
        }
    };

    cache.insert((stone, iterations), value);
    value
}

pub fn part1(input: &str) -> PuzzleResult {
    let mut blink_cache: HashMap<(i64, i32), i64> = HashMap::new();

    Ok(parse_input(input)
        .into_iter()
        .map(|stone| blink(stone, 25, &mut blink_cache))
        .sum::<i64>() as usize)
}

pub fn part2(input: &str) -> PuzzleResult {
    let mut blink_cache: HashMap<(i64, i32), i64> = HashMap::new();

    Ok(parse_input(input)
        .into_iter()
        .map(|stone| blink(stone, 75, &mut blink_cache))
        .sum::<i64>() as usize)
}
