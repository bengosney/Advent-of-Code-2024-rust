use std::iter::zip;

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

pub fn part1(input: &str) -> Result<i32, &'static str> {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    Ok(zip(left, right).map(|(l, r)| (r - l).abs()).sum())
}

pub fn part2(input: &str) -> Result<i32, &'static str> {
    let (left, right) = parse_input(input);

    let count_items = |what: i32| right.iter().filter(|&r| *r == what).count();

    Ok(left.iter().map(|l| l * count_items(*l) as i32).sum())
}
