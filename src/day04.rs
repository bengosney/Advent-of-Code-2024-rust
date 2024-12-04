use std::collections::HashMap;

#[cfg(test)]
const EXAMPLE_INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

#[test]
fn test_part1_example() {
    let part1 = part1(EXAMPLE_INPUT);
    assert_eq!(part1, Result::Ok(18));
}

#[test]
fn test_part2_example() {
    let part2 = part2(EXAMPLE_INPUT);
    assert_eq!(part2, Result::Ok(9));
}

fn puzzle_to_grid(puzzle: &str) -> HashMap<(isize, isize), char> {
    let mut grid = HashMap::new();

    for (y, line) in puzzle.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            grid.insert((x as isize, y as isize), cell);
        }
    }

    grid
}

pub fn part1(input: &str) -> Result<i32, &'static str> {
    let grid = puzzle_to_grid(input);

    let forwords = ['X', 'M', 'A', 'S'];
    let backwords = ['S', 'A', 'M', 'X'];

    let grid_get = |x, y| *grid.get(&(x, y)).unwrap_or(&' ');

    let directions = [(1, 0), (0, 1), (1, 1), (1, -1)];

    let mut total = 0;
    for &(x, y) in grid.keys() {
        for &(dx, dy) in &directions {
            let to_check: Vec<char> = (0..4)
                .map(|i| grid_get(x.wrapping_add(i * dx), y.wrapping_add(i * dy)))
                .collect();

            if to_check == forwords || to_check == backwords {
                total += 1;
            }
        }
    }

    Ok(total)
}

pub fn part2(input: &str) -> Result<i32, &'static str> {
    let grid = puzzle_to_grid(input);

    let grid_get = |x, y| *grid.get(&(x, y)).unwrap_or(&' ');

    let mut found = 0;

    const PATTERNS: [[char; 5]; 5] = [
        ['M', 'S', 'A', 'M', 'S'],
        ['S', 'M', 'A', 'S', 'M'],
        ['M', 'S', 'A', 'M', 's'],
        ['S', 'S', 'A', 'M', 'M'],
        ['M', 'M', 'A', 'S', 'S'],
    ];

    for &(x, y) in grid.keys() {
        let selected = [
            grid_get(x, y),
            grid_get(x + 2, y),
            grid_get(x + 1, y + 1),
            grid_get(x, y + 2),
            grid_get(x + 2, y + 2),
        ];

        if PATTERNS.iter().any(|&pattern| pattern == selected) {
            found += 1;
        }
    }

    Ok(found)
}
