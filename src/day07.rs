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
    answer: usize,
    values: Vec<u32>,
}

fn parse_input(input: &str) -> Vec<Calculation> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let answer = parts.next().unwrap().parse().unwrap();
            let values = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .rev()
                .collect();
            Calculation { answer, values }
        })
        .collect()
}

type Operation = fn(usize, usize) -> usize;

fn calculate(values: &Vec<u32>, operations: &Vec<Operation>) -> Vec<usize> {
    if values.len() == 1 {
        return vec![values[0] as usize];
    }

    let capacity = operations.len().pow(values.len() as u32 - 1);
    let mut result = Vec::with_capacity(capacity);

    calculate(&values[1..].to_vec(), operations)
        .iter()
        .flat_map(|right| operations.iter().map(|op| op(*right, values[0] as usize)))
        .for_each(|val| result.push(val));

    result
}

pub fn part1(input: &str) -> PuzzleResult {
    let calculations = parse_input(input);
    let mut total = 0;

    let operations = vec![|a: usize, b: usize| a + b, |a: usize, b: usize| a * b];

    for calculation in calculations {
        let values = calculation.values;
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
        |a: usize, b: usize| (a + b) as usize,
        |a: usize, b: usize| (a * b) as usize,
        |a: usize, b: usize| format!("{}{}", a, b).parse::<usize>().unwrap(),
    ];

    for calculation in calculations {
        let values = calculation.values;
        let calculated = calculate(&values, &operations);
        if calculated.iter().any(|&x| x == calculation.answer) {
            total += calculation.answer;
        }
    }

    Ok(total as usize)
}
