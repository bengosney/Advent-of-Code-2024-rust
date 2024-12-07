use crate::utils::PuzzleResult;

#[cfg(test)]
const EXAMPLE_INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

#[test]
fn test_part1_example() {
    let part1 = part1(EXAMPLE_INPUT);
    assert_eq!(part1, Result::Ok(3749));
}

#[test]
fn test_part2_example() {
    let part2 = part2(EXAMPLE_INPUT);
    assert_eq!(part2, Result::Ok(11387));
}

struct Calculation {
    answer: isize,
    values: Vec<isize>,
}

fn parse_input(input: &str) -> Vec<Calculation> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let answer = parts.next().unwrap().parse::<isize>().unwrap();
            let values = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .collect();
            Calculation { answer, values }
        })
        .collect()
}

fn calculate(values: &Vec<isize>, operations: &Vec<fn(isize, isize)->isize>) -> Vec<isize> {
    if values.len() == 1 {
        return values.clone();
    }

    let left = values[0];

    calculate(&values[1..].to_vec(), operations)
        .iter()
        .flat_map(|right| operations.iter().map(|op| op(*right, left)))
        .collect()
}

pub fn part1(input: &str) -> PuzzleResult {
    let calculations = parse_input(input);
    let mut total = 0;

    let operations = vec![|a:isize, b:isize| a + b, |a:isize, b:isize| a * b];

    for calculation in calculations {
        let mut values = calculation.values;
        values.reverse();
        let calculated = calculate(&values, &operations);
        if calculated.iter().any(|&x| x == calculation.answer) {
            total += calculation.answer;
        }
    }

    Ok(total as usize)
}

pub fn part2(input: &str) -> PuzzleResult {
    let calculations = parse_input(input);
    let mut total = 0;

    let operations = vec![
        |a:isize, b:isize| a + b, 
        |a:isize, b:isize| a * b,
        |a:isize, b:isize| format!("{}{}", a, b).parse::<isize>().unwrap(),
    ];

    for calculation in calculations {
        let mut values = calculation.values;
        values.reverse();
        let calculated = calculate(&values, &operations);
        if calculated.iter().any(|&x| x == calculation.answer) {
            total += calculation.answer;
        }
    }

    Ok(total as usize)
}
