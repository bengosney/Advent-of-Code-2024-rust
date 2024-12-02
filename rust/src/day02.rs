#[cfg(test)]
const EXAMPLE_INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

#[test]
fn test_part1_example() {
    let part1 = part1(EXAMPLE_INPUT);
    assert_eq!(part1, Result::Ok(2));
}

#[test]
fn test_part2_example() {
    let part2 = part2(EXAMPLE_INPUT);
    assert_eq!(part2, Result::Ok(4));
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn check_row(row: &Vec<i32>) -> bool {
    let mut row_dir = 0;
    for i in 0..(row.len()-1) {
        let dir = (row[i] < row[i+1]) as i32 - (row[i] > row[i+1]) as i32;
        if ![0, dir].contains(&row_dir) {
            return false;
        }
        row_dir = dir;

        match (row[i] - row[i+1]).abs() {
            1 | 2 | 3 => continue,
            _ => return false,
        }
    }

    return true
}

pub fn part1(input: &str) -> Result<i32, &'static str> {
    let rows = parse_input(input);
    let count = rows.iter().filter(|row| check_row(row)).count() as i32;
    Ok(count)
}

pub fn part2(input: &str) -> Result<i32, &'static str> {
    let rows = parse_input(input);
    let mut count = 0;
    for row in rows.iter() {
        if check_row(row) {
            count += 1;
            continue;
        }
        for i in 0..row.len() {
            let mut row = row.clone();
            row.remove(i);
            if check_row(&row) {
                count += 1;
                break;
            }
        }
    }
    
    Ok(count)
}
