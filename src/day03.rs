use regex::Regex;

#[cfg(test)]
const EXAMPLE_INPUT_1: &str =
    r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

#[cfg(test)]
const EXAMPLE_INPUT_2: &str =
    r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

#[test]
fn test_part1_example() {
    let part1 = part1(EXAMPLE_INPUT_1);
    assert_eq!(part1, Result::Ok(161));
}

#[test]
fn test_part2_example() {
    let part2 = part2(EXAMPLE_INPUT_2);
    assert_eq!(part2, Result::Ok(48));
}

pub fn part1(input: &str) -> Result<i32, &'static str> {
    let regex = Regex::new(r"(?m)mul\((\d+),(\d+)\)").unwrap();

    let total: i32 = regex
        .captures_iter(input)
        .map(|cap| {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            a * b
        })
        .sum();

    Ok(total)
}

pub fn part2(input: &str) -> Result<i32, &'static str> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();

    let mut working: bool = true;
    let total: i32 = regex
        .find_iter(input)
        .map(|cap| match cap.as_str() {
            "do()" => {
                working = true;
                0
            }
            "don't()" => {
                working = false;
                0
            }
            operation => {
                let result = Regex::new(r"mul\((\d+),(\d+)\)")
                    .unwrap()
                    .captures(operation)
                    .unwrap();
                let a = result[1].parse::<i32>().unwrap();
                let b = result[2].parse::<i32>().unwrap();
                (a * b) * working as i32
            }
        })
        .sum();

    Ok(total)
}
