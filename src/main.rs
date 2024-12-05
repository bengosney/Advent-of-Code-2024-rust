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
/*[[[end]]] (checksum: 96e175fc8ba9ede1c96d85076f4f131d) */

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
        /*[[[end]]] (checksum: 2dacd729cbc827311b402e56fdb15fc8)*/
        _ => panic!("Unknown day"),
    };

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
