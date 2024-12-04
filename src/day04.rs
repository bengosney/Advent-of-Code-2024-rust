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
    puzzle
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, cell)| ((x as isize, y as isize), cell))
        })
        .collect()
}

pub fn part1(input: &str) -> Result<i32, &'static str> {
    let grid = puzzle_to_grid(input);

    const FORWORDS: [char; 4] = ['X', 'M', 'A', 'S'];
    const BACKWORDS: [char; 4] = ['S', 'A', 'M', 'X'];

    let grid_get = |x, y| *grid.get(&(x, y)).unwrap_or(&' ');

    const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (1, 1), (1, -1)];

    Ok(grid
        .keys()
        .flat_map(|&(x, y)| {
            DIRECTIONS.iter().map(move |&(dx, dy)| {
                let to_check: Vec<char> =
                    (0..4).map(|i| grid_get(x + i * dx, y + i * dy)).collect();

                match to_check == FORWORDS || to_check == BACKWORDS {
                    true => 1,
                    false => 0,
                }
            })
        })
        .sum())
}

pub fn part2(input: &str) -> Result<i32, &'static str> {
    const PATTERNS: [[char; 5]; 5] = [
        ['M', 'S', 'A', 'M', 'S'],
        ['S', 'M', 'A', 'S', 'M'],
        ['M', 'S', 'A', 'M', 's'],
        ['S', 'S', 'A', 'M', 'M'],
        ['M', 'M', 'A', 'S', 'S'],
    ];

    let grid = puzzle_to_grid(input);

    let select = |x, y| {
        let grid_get = |x, y| *grid.get(&(x, y)).unwrap_or(&' ');
        [
            grid_get(x, y),
            grid_get(x + 2, y),
            grid_get(x + 1, y + 1),
            grid_get(x, y + 2),
            grid_get(x + 2, y + 2),
        ]
    };

    Ok(grid
        .keys()
        .map(|&(x, y)| {
            let selected = select(x, y);
            match PATTERNS.iter().any(|&pattern| pattern == selected) {
                true => 1,
                false => 0,
            }
        })
        .sum())
}
