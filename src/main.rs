pub mod point;
mod utils;

/* [[[cog
import cog
from pathlib import Path
for day in sorted([p.name.replace(".rs", "") for p in list(Path("./src").glob("day*.rs"))]):
   cog.outl(f"mod {day};")
]]]*/
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day10;
/*[[[end]]] (checksum: 2c8daae1c0adf95daab3cb1d976f05ad) */

fn read_input(day: &str) -> String {
    let path = format!("./inputs/{}_{}.txt", &day[..3], &day[3..]);
    match std::fs::read_to_string(path) {
        Ok(input) => input,
        Err(e) => panic!("Error reading input for {}: {}", day, e),
    }
}

fn main() {
    let day = std::env::args().nth(1).expect("Please provide a day");
    let input = read_input(&day);

    let (part1, part2) = match day.as_str() {
        /* [[[cog
        import cog
        from pathlib import Path
        for day in sorted([p.name.replace(".rs", "") for p in list(Path("./src").glob("day*.rs"))]):
            cog.outl(f'"{day}" => ({day}::part1(&input), {day}::part2(&input)),')
        ]]]*/
        "day01" => (day01::part1(&input), day01::part2(&input)),
        "day02" => (day02::part1(&input), day02::part2(&input)),
        "day03" => (day03::part1(&input), day03::part2(&input)),
        "day04" => (day04::part1(&input), day04::part2(&input)),
        "day05" => (day05::part1(&input), day05::part2(&input)),
        "day06" => (day06::part1(&input), day06::part2(&input)),
        "day07" => (day07::part1(&input), day07::part2(&input)),
        "day08" => (day08::part1(&input), day08::part2(&input)),
        "day10" => (day10::part1(&input), day10::part2(&input)),
        /*[[[end]]] (checksum: 5cb9ac451397f20b106e04a7cee31e93)*/
        _ => panic!("Unknown day"),
    };

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
