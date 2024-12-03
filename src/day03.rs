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
    let main_regex = Regex::new(r"mul\(\d+,\d+\)|(do\(\))|(don't\(\))").unwrap();
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut working: bool = true;
    let mut total: i32 = 0;

    for cap in main_regex.find_iter(input) {
        match cap.as_str() {
            "do()" => working = true,
            "don't()" => working = false,
            operation if working => {
                if let Some(result) = mul_regex.captures(operation) {
                    let a = result[1].parse::<i32>().unwrap();
                    let b = result[2].parse::<i32>().unwrap();
                    total += a * b;
                }
            }
            _ => {}
        }
    }

    Ok(total)
}
