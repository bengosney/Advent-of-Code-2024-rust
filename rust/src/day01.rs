use std::iter::zip;

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

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let numbers: Vec<i32> =
            line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        left.push(numbers[0]);
        right.push(numbers[1]);
    }
    left.sort();
    right.sort();
    
    (left, right)
}

pub fn part1(input: &str) -> Result<i32, &'static str> {
    let mut sum = 0;
    let (left, right) = parse_input(input);
    for (l,r) in zip(left, right) {
        sum += (r - l).abs();
    }
    Ok(sum)
}
